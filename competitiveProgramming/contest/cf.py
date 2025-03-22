import os
import subprocess
import sys

import pyperclip

# File to store the saved URLs
LINKS_FILE = "saved_links.txt"


# Function to save a URL for future use
def add_link(url):
    with open(LINKS_FILE, "a", encoding="utf-8") as file:
        file.write(url + "\n")


# Function to load a problem (fetch HTML and run samples.py)
def load_problem(problem_id):
    # Read the saved URL for the problem
    if not os.path.exists(LINKS_FILE):
        print("No saved URLs found. Please run 'cf add <link>' first.")
        return

    with open(LINKS_FILE, "r", encoding="utf-8") as file:
        links = file.readlines()

    # Find the URL for the given problem_id
    if not links:
        print("No URLs saved. Please add a URL first.")
        return

    base_url = links[-1].strip()  # Use the first saved URL

    url = f"{base_url}/problem/{problem_id}"

    print(f"Using URL {url} for problem {problem_id}")

    # Run extractHtml.py to fetch the HTML content
    subprocess.run([sys.executable, "extractHtml.py", url], check=True)

    # Run samples.py to extract input and output data
    subprocess.run([sys.executable, "samples.py"], check=True)


# Function to check if the C++ code works as expected
def check_problem():
    input_file = "input.txt"
    expected_file = "expected.txt"
    cpp_script = "run.sh"

    # Run the run.sh script with the input from input.txt
    print(f"Running script {cpp_script} with input from {input_file}...")
    with open(input_file, "r", encoding="utf-8") as input_data:
        process = subprocess.run(
            ["bash", cpp_script],
            input=input_data.read(),
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )


def submit_problem(problem_id):
    # Read the saved URL for the problem
    if not os.path.exists(LINKS_FILE):
        print("No saved URLs found. Please run 'cf add <link>' first.")
        return

    with open(LINKS_FILE, "r", encoding="utf-8") as file:
        links = file.readlines()

    # Find the URL for the given problem_id
    if not links:
        print("No URLs saved. Please add a URL first.")
        return

    base_url = links[-1].strip()  # Use the first saved URL
    submit_url = f"{base_url}/submit/{problem_id}"

    # Open the submit URL in LibreWolf
    subprocess.run(["librewolf", submit_url])

    # Copy the content of workspace.cpp to clipboard
    workspace_file_path = "workspace.cpp"

    if not os.path.exists(workspace_file_path):
        print(f"{workspace_file_path} not found. Make sure the file exists.")
        return

    # Read the content of workspace.cpp
    with open(workspace_file_path, "r", encoding="utf-8") as file:
        code = file.read()

    # Copy the content to clipboard
    pyperclip.copy(code)


# Main function to execute different commands


def main():
    if len(sys.argv) < 2:
        print("Usage: python cf.py <command> <problem_id> [url]")
        return

    command = sys.argv[1]

    if command == "add":
        if len(sys.argv) < 3:
            print("Please provide the URL to save.")
            return
        # Add the URL to saved links
        add_link(sys.argv[2])

    elif command == "load":
        if len(sys.argv) < 3:
            print("Please provide the problem ID.")
            return
        problem_id = sys.argv[2]
        # Load the problem and save the input/output
        load_problem(problem_id)

    elif command == "check":
        # Check the problem by running the script and comparing output
        check_problem()

    elif command == "submit":
        if len(sys.argv) < 3:
            print("Please provide the problem ID to submit.")
            return
        problem_id = sys.argv[2]
        # Submit the problem using LibreWolf
        submit_problem(problem_id)

    else:
        print(f"Unknown command {command}. Use 'cf add', 'cf load', or 'cf check'.")


if __name__ == "__main__":
    main()
