package main

import (
    // "bufio"
    "encoding/json"
    "fmt"
    "github.com/atotto/clipboard"
    "github.com/PuerkitoBio/goquery"
    // "io"
    "io/ioutil"
    "net/http"
    "os"
    "os/exec"
    // "path/filepath"
    "regexp"
    "strings"
    "time"
)

const LINKS_FILE = "saved_links.txt"

// --------------------- ADD LINK ---------------------

func addLink(url string) {
    f, err := os.OpenFile(LINKS_FILE, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
    if err != nil {
        fmt.Println("❌ Could not open saved_links.txt:", err)
        return
    }
    defer f.Close()

    if _, err = f.WriteString(url + "\n"); err != nil {
        fmt.Println("❌ Could not write to saved_links.txt:", err)
        return
    }
    fmt.Println("✅ Added link:", url)
}

// --------------------- LOAD PROBLEM ---------------------

func loadProblem(url string) {
    if url == "" {
        fmt.Println("No URL provided.")
        return
    }

    // Last two segments (like 2055/C)
    parts := strings.Split(strings.TrimSpace(url), "/")
    segCount := len(parts)
    if segCount < 2 {
        fmt.Println("URL seems invalid.")
        return
    }
    urlSegment := strings.Join(parts[segCount-2:], "/")

    // Write segment to saved_links.txt
    err := ioutil.WriteFile(LINKS_FILE, []byte(urlSegment+"\n"), 0644)
    if err != nil {
        fmt.Println("❌ Could not write to saved_links.txt:", err)
        return
    }

    // Fetch the page
    client := &http.Client{}
    req, _ := http.NewRequest("GET", url, nil)
    req.Header.Set("User-Agent", "Mozilla/5.0")
    resp, err := client.Do(req)
    if err != nil {
        fmt.Println("❌ HTTP error:", err)
        return
    }
    defer resp.Body.Close()

    if resp.StatusCode != 200 {
        fmt.Printf("❌ Non-200 response: %d\n", resp.StatusCode)
        return
    }

    doc, err := goquery.NewDocumentFromReader(resp.Body)
    if err != nil {
        fmt.Println("❌ goquery parse error:", err)
        return
    }

    // We'll replicate your logic:
    // 1) Find all div.test-example-line-<idx> -> group lines by index
    // 2) Find all "Output" -> next <pre> -> outputs
    reLine := regexp.MustCompile(`test-example-line-(\d+)`)

    inputsMap := make(map[int][]string)

    // find all divs with class ~ test-example-line
    doc.Find("div").Each(func(i int, s *goquery.Selection) {
        class, _ := s.Attr("class")
        if strings.Contains(class, "test-example-line") {
            matches := reLine.FindStringSubmatch(class)
            if len(matches) == 2 {
                idx := matches[1] // string of the number
                // convert to int
                // but let's store them in a map using the int
                // We can parse it:
                // We'll ignore parse error if any
                var iidx int
                fmt.Sscanf(idx, "%d", &iidx)
                text := strings.TrimSpace(s.Text())
                inputsMap[iidx] = append(inputsMap[iidx], text)
            }
        }
    })

    // find all "Output" div.title => next <pre>
    var outputs []string
    doc.Find("div.title").Each(func(i int, s *goquery.Selection) {
        if strings.TrimSpace(s.Text()) == "Output" {
            pre := s.NextFiltered("pre")
            if pre != nil {
                outputs = append(outputs, strings.TrimSpace(pre.Text()))
            }
        }
    })

    // write input.txt
    inFile, err := os.Create("input.txt")
    if err != nil {
        fmt.Println("❌ Could not create input.txt:", err)
        return
    }
    defer inFile.Close()

    var idxList []int
    for k := range inputsMap {
        idxList = append(idxList, k)
    }
    // Sort them
    // we can do a bubble or just create a small function
    // but let's do "sort.Ints"
    importSortInts(idxList)

    for _, idxVal := range idxList {
        lines := inputsMap[idxVal]
        for _, line := range lines {
            inFile.WriteString(line + "\n")
        }
    }

    // write expected.txt
    outFile, err := os.Create("expected.txt")
    if err != nil {
        fmt.Println("❌ Could not create expected.txt:", err)
        return
    }
    defer outFile.Close()

    for _, outLine := range outputs {
        outFile.WriteString(outLine + "\n")
    }

    fmt.Println("✅ Test cases extracted.")
}

// custom int sorting without importing a big package
func importSortInts(a []int) {
    for i := 0; i < len(a); i++ {
        for j := i + 1; j < len(a); j++ {
            if a[i] > a[j] {
                a[i], a[j] = a[j], a[i]
            }
        }
    }
}

// --------------------- COMPILE & RUN ---------------------

func compileCpp() bool {
    cmd := exec.Command("g++", "-std=c++20", "-O2", "-o", "workspace", "workspace.cpp")
    err := cmd.Run()
    return err == nil
}

func runCpp() bool {
    if !compileCpp() {
        fmt.Println("❌ Compilation failed.")
        return false
    }

    if _, err := os.Stat("input.txt"); os.IsNotExist(err) {
        fmt.Println("❌ input.txt not found.")
        return false
    }

    // run ./workspace with input.txt
    inFile, err := os.Open("input.txt")
    if err != nil {
        fmt.Println("❌ Could not open input.txt:", err)
        return false
    }
    defer inFile.Close()

    cmd := exec.Command("./workspace")
    cmd.Stdin = inFile
    outBytes, err := cmd.Output()
    if err != nil {
        fmt.Println("❌ Runtime error:", err)
        return false
    }
    outputLines := strings.Split(string(outBytes), "\n")
    for i := range outputLines {
        outputLines[i] = strings.TrimSpace(outputLines[i])
    }

    if _, err := os.Stat("expected.txt"); os.IsNotExist(err) {
        fmt.Println("❌ expected.txt not found.")
        return false
    }
    expBytes, _ := ioutil.ReadFile("expected.txt")
    expLines := strings.Split(string(expBytes), "\n")

    var filteredExp []string
    for _, l := range expLines {
        line := strings.TrimSpace(l)
        if line != "" {
            filteredExp = append(filteredExp, line)
        }
    }

    fmt.Println("\n📤 Output:")
    for _, line := range outputLines {
        fmt.Println(line)
    }

    fmt.Println("\n✅ Checking output...\n")
    allPassed := true
    maxLen := len(outputLines)
    if len(filteredExp) > maxLen {
        maxLen = len(filteredExp)
    }

    for i := 0; i < maxLen; i++ {
        var outLine, expLine string
        if i < len(outputLines) {
            outLine = outputLines[i]
        }
        if i < len(filteredExp) {
            expLine = filteredExp[i]
        }
        if outLine == expLine {
            fmt.Printf("\033[32m+ Case %d: %s\033[0m\n", i+1, outLine)
        } else {
            fmt.Printf("\033[31m- Case %d: %s\033[0m\n", i+1, outLine)
            fmt.Printf("\033[33m  Expected: %s\033[0m\n", expLine)
            allPassed = false
        }
    }

    if allPassed {
        fmt.Println("\n\033[32m✅ Passed\033[0m")
    } else {
        fmt.Println("\n\033[31m❌ Wrong Answer\033[0m")
    }
    return allPassed
}

// --------------------- CHECK PROBLEM ---------------------

func checkProblem() {
    if runCpp() {
        submitProblem()
    }
}

// --------------------- SUBMIT PROBLEM ---------------------

func submitProblem() {
    if _, err := os.Stat(LINKS_FILE); os.IsNotExist(err) {
        fmt.Println("❌ No saved URLs.")
        return
    }

    data, _ := ioutil.ReadFile(LINKS_FILE)
    urlFrag := strings.TrimSpace(string(data))
    if urlFrag == "" {
        fmt.Println("❌ Empty saved URL.")
        return
    }

    // open with librewolf
    subURL := fmt.Sprintf("https://codeforces.com/problemset/submit/%s", urlFrag)
    exec.Command("librewolf", subURL).Start()

    // copy workspace.cpp
    if _, err := os.Stat("workspace.cpp"); os.IsNotExist(err) {
        fmt.Println("❌ workspace.cpp not found.")
        return
    }
    codeBytes, _ := ioutil.ReadFile("workspace.cpp")
    clipboard.WriteAll(string(codeBytes))
    fmt.Println("✅ Copied workspace.cpp to clipboard.")
}

// --------------------- FRIENDS ---------------------

func friendsCmd() {
    if _, err := os.Stat(LINKS_FILE); os.IsNotExist(err) {
        fmt.Println("❌ No saved URLs.")
        return
    }
    data, _ := ioutil.ReadFile(LINKS_FILE)
    line := strings.TrimSpace(string(data))
    parts := strings.Split(line, "/")
    contestID := parts[0]
    // open link
    frURL := fmt.Sprintf("https://codeforces.com/contest/%s/standings/friends/true", contestID)
    exec.Command("librewolf", frURL).Start()
}

// --------------------- CHECK LAST SUBMISSION ---------------------

func checkLastSubmission() {
    handle := "worthNothing"
    apiURL := fmt.Sprintf("https://codeforces.com/api/user.status?handle=%s&from=1&count=1", handle)

    resp, err := http.Get(apiURL)
    if err != nil {
        fmt.Println("❌ API error:", err)
        return
    }
    defer resp.Body.Close()

    var result struct {
        Status string `json:"status"`
        Comment string `json:"comment"`
        Result []map[string]interface{} `json:"result"`
    }

    dec := json.NewDecoder(resp.Body)
    if err := dec.Decode(&result); err != nil {
        fmt.Println("❌ JSON decode error:", err)
        return
    }

    if result.Status != "OK" || len(result.Result) == 0 {
        fmt.Println("❌ API error or no submissions.")
        return
    }

    sub := result.Result[0]
    prob, _ := sub["problem"].(map[string]interface{})
    contestId, _ := prob["contestId"].(float64)
    index, _ := prob["index"].(string)
    name, _ := prob["name"].(string)
    verdict, _ := sub["verdict"].(string)
    passedTestCount, _ := sub["passedTestCount"].(float64)
    timeConsumed, _ := sub["timeConsumedMillis"].(float64)
    memoryUsed, _ := sub["memoryConsumedBytes"].(float64)

    fmt.Printf("📘 Problem: %.0f%s - %s\n", contestId, index, name)
    if verdict == "" {
        verdict = "N/A"
    }
    fmt.Printf("🧪 Verdict: %s\n", verdict)
    fmt.Printf("✅ Passed: %.0f\n", passedTestCount)
    fmt.Printf("⚡ Time: %.0f ms\n", timeConsumed)
    fmt.Printf("📦 Memory: %.0f bytes\n", memoryUsed)
}

// --------------------- SHOW UPCOMING REGULAR CONTESTS ---------------------

func showUpcomingRegularContests() {
    apiURL := "https://codeforces.com/api/contest.list?gym=false"
    resp, err := http.Get(apiURL)
    if err != nil {
        fmt.Println("❌ HTTP error:", err)
        return
    }
    defer resp.Body.Close()

    var data struct {
        Status string `json:"status"`
        Comment string `json:"comment"`
        Result []map[string]interface{} `json:"result"`
    }
    if err := json.NewDecoder(resp.Body).Decode(&data); err != nil {
        fmt.Println("❌ JSON decode error:", err)
        return
    }
    if data.Status != "OK" {
        fmt.Println("❌ Failed:", data.Comment)
        return
    }

    // Filter upcoming
    upcoming := []map[string]interface{}{}
    for _, c := range data.Result {
        if c["phase"] == "BEFORE" {
            upcoming = append(upcoming, c)
        }
    }
    if len(upcoming) == 0 {
        fmt.Println("📭 No upcoming contests.")
        return
    }

    // Sort by startTimeSeconds
    for i := 0; i < len(upcoming); i++ {
        for j := i+1; j < len(upcoming); j++ {
            si := upcoming[i]["startTimeSeconds"].(float64)
            sj := upcoming[j]["startTimeSeconds"].(float64)
            if si > sj {
                upcoming[i], upcoming[j] = upcoming[j], upcoming[i]
            }
        }
    }

    fmt.Println("\n📅 Upcoming Contests (IST):\n")
    for _, c := range upcoming {
        name, _ := c["name"].(string)
        startF, _ := c["startTimeSeconds"].(float64)
        durationF, _ := c["durationSeconds"].(float64)

        // Convert to IST
        startSec := int64(startF)
        durHours := int64(durationF / 3600)
        // define IST offset
        ist := time.FixedZone("IST", 5*3600+30*60)
        startTime := time.Unix(startSec, 0).UTC().In(ist)

        fmt.Printf("📌 %s | 🕒 %s IST | ⏱️ %dh\n",
            name,
            startTime.Format("2006-01-02 15:04"),
            durHours,
        )
    }
}

// --------------------- MAIN ---------------------

func main() {
    if len(os.Args) < 2 {
        fmt.Println("Usage: go run cf.go <command> [args]")
        return
    }
    cmd := os.Args[1]
    var arg string
    if len(os.Args) > 2 {
        arg = os.Args[2]
    }

    switch cmd {
    case "add":
        if arg == "" {
            fmt.Println("❌ Missing URL for 'add'")
            return
        }
        addLink(arg)

    case "load":
        if arg == "" {
            fmt.Println("❌ Missing URL for 'load'")
            return
        }
        loadProblem(arg)

    case "check":
        checkProblem()

    case "submit":
        submitProblem()

    case "friends":
        friendsCmd()

    case "last":
        checkLastSubmission()

    case "contest":
        showUpcomingRegularContests()

    default:
        fmt.Printf("❓ Unknown or incomplete command '%s'\n", cmd)
    }
}
