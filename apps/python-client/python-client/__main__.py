import requests


def main():
    url = "http://127.0.0.1:3030/sayhi"
    response = requests.get(url)
    if response.status_code == 200:
        print("Response from server:", response.json())
    else:
        print("Failed to connect:", response.status_code)


if __name__ == "__main__":
    main()
