import sys
import markovify

if __name__ == "__main__":

    data = sys.stdin.read()
    model = markovify.Text(data, state_size=3)

    try:
        while True:
            print(model.make_sentence(), end="")
    except KeyboardInterrupt:
        print()

