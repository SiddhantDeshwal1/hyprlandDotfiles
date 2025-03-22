
#!/bin/bash

# Compile the C++ code
g++ -O3 -o workspace workspace.cpp
# clang++ -O3 -o workspace workspace.cpp

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed. Exiting."
    exit 1
fi

# Run the compiled program and store the output in a temporary file
./workspace > output.txt

# Trim spaces from the start and end of each line in the program output
sed -i 's/^[ \t]*//;s/[ \t]*$//' output.txt

# Trim spaces and remove leading empty lines from expected output
sed -i '/^[[:space:]]*$/d' expected.txt  # Remove all empty lines
sed -i 's/^[ \t]*//;s/[ \t]*$//' expected.txt  # Trim spaces from the start and end

# Display the program output
echo
echo "Program Output:"
cat output.txt

# Compare the output with the expected output
echo ""
echo "Checking if output matches expected..."

# Read both files line by line and compare them
output_lines=$(cat output.txt)
expected_lines=$(cat expected.txt)

output_array=()
expected_array=()

while IFS= read -r line; do
    output_array+=("$line")
done <<< "$output_lines"

while IFS= read -r line; do
    expected_array+=("$line")
done <<< "$expected_lines"

len_output=${#output_array[@]}
len_expected=${#expected_array[@]}
max_len=$((len_output > len_expected ? len_output : len_expected))

# Initialize verdict
all_cases_passed=true

for i in $(seq 0 $((max_len - 1))); do
    output_case="${output_array[i]}"
    expected_case="${expected_array[i]}"

    if [ "$output_case" == "$expected_case" ]; then
        echo -e "\x1b[32m+ Case $((i + 1)): $output_case\x1b[0m"  # Green for correct case
    else
        echo -e "\x1b[31m- Case $((i + 1)): $output_case\x1b[0m"  # Red for incorrect case
        echo -e "\x1b[33m  Expected: $expected_case\x1b[0m"  # Yellow for expected case
        all_cases_passed=false
    fi
done

# Simple verdict
if [ "$all_cases_passed" = true ]; then
    echo -e "\n\x1b[32mVerdict: ✅ Passed\x1b[0m"
else
    echo -e "\n\x1b[31mVerdict: ❌ Wrong Answer\x1b[0m"
fi

# Clean up
rm output.txt
