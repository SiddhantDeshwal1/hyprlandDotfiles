from bs4 import BeautifulSoup

# Open the HTML file
with open("data.html", "r", encoding="utf-8") as html_file:
    content = html_file.read()

    # Parse the HTML content
    soup = BeautifulSoup(content, "lxml")

    # Open files to save input and expected output
    input_file = open("input.txt", "w", encoding="utf-8")
    expected_file = open("expected.txt", "w", encoding="utf-8")

    # Find all divs with class 'test-example-line' (input data)
    tags = soup.find_all("div", class_="test-example-line")

    # Save the content of each tag for input data to input.txt
    for tag in tags:
        input_file.write(tag.get_text().strip() + "\n")

    # Find the div with class 'title' that contains the text "Output"
    title_div = soup.find("div", class_="title", string="Output")

    # Find the next <pre> tag after the "Output" div (output data)
    if title_div:
        pre_tag = title_div.find_next("pre")
        if pre_tag:
            # Extract the text inside <pre> and save to expected.txt
            output_data = pre_tag.get_text().strip()
            expected_file.write(output_data + "\n")
        else:
            print("No <pre> tag found after the 'Output' div.")
    else:
        print("No 'Output' div found.")

    # Close the files
    input_file.close()
    expected_file.close()

    print("Input and output data saved to input.txt and expected.txt respectively.")
