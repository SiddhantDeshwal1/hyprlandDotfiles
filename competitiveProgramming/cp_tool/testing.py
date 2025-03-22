from youtubesearchpython import VideosSearch


def get_top_5_youtube_links(query):
    videos_search = VideosSearch(
        query, limit=10
    )  # Fetch more in case many are unrelated
    results = videos_search.result().get("result", [])

    # Extract round number from query
    round_number = None
    for word in query.split():
        if word.isdigit():
            round_number = word
            break

    if not round_number:
        return ["No valid round number found in query."]

    filtered = []
    for video in results:
        title = video.get("title", "").lower()
        link = video.get("link", "")
        if round_number in title:
            filtered.append(f"{title}: {link}")
        if len(filtered) == 5:
            break

    return filtered if filtered else [f"No matching videos for round {round_number}."]


query = "codeforces round 1009 (div 3) Editorial"
top_links = get_top_5_youtube_links(query)
for link in top_links:
    print(link)
