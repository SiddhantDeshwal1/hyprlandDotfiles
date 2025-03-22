import os
import sys

import requests


def fetch_html(url):
    # Headers to mimic a browser request
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"
    }

    # Send a GET request to fetch the HTML content with headers
    response = requests.get(url, headers=headers)

    # Check if the request was successful
    if response.status_code == 200:
        html_content = response.text

        # Get the current directory where the script is located
        current_directory = os.getcwd()

        # Define the path for saving the HTML file in the current directory
        file_path = os.path.join(current_directory, "data.html")

        # Save the HTML content to the file
        with open(file_path, "w", encoding="utf-8") as file:
            file.write(html_content)

        print(f"HTML content saved to {file_path}")
    else:
        print(f"Failed to retrieve the page. Status code: {response.status_code}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python extractHtml.py <URL>")
    else:
        url = sys.argv[1]
        fetch_html(url)
