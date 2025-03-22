# import requests
#
#
# def get_codeforces_rating(username):
#     url = f"https://codeforces.com/api/user.info?handles={username}"
#     res = requests.get(url).json()
#     if res["status"] == "OK":
#         return res["result"][0]["rating"]
#     return None
#
#
# # Example:
# print("Codeforces:", get_codeforces_rating("tourist"))
#
#
#
# import requests
#
# def get_leetcode_rating(username):
#     query = {
#         "query": """
#         query getUserContestRanking($username: String!) {
#           userContestRanking(username: $username) {
#             rating
#           }
#         }
#         """,
#         "variables": {"username": username}
#     }
#     res = requests.post("https://leetcode.com/graphql", json=query)
#     data = res.json()
#     rating = data["data"]["userContestRanking"]
#     return rating["rating"] if rating else None
#
# # Example:
# print("LeetCode:", get_leetcode_rating("NeetCode"))
#
#
#
# import requests
# from bs4 import BeautifulSoup
#
# def get_codechef_rating(username):
#     url = f"https://www.codechef.com/users/{username}"
#     res = requests.get(url)
#     soup = BeautifulSoup(res.text, "html.parser")
#     rating_tag = soup.find("div", class_="rating-number")
#     return rating_tag.text.strip() if rating_tag else None
#
# # Example:
# print("CodeChef:", get_codechef_rating("tourist"))
