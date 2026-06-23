def count_words(filename):

    word_count = {}

    with open(filename, 'r') as file:
        for line in file:
            for word in line.split():
                word_count[word] = word_count.get(word, 0) + 1

    return word_count

def main():
    counts = count_words("C:\pravin\Projects\pravinrepo\python-rust-go\words.txt")
    print(counts)

if __name__ == "__main__":
    main()