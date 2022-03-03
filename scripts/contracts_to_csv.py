import json
import sys
import pandas



def read_json(filename: str) -> dict:

    try:
        with open(filename, "r") as f:
            data = json.loads(f.read())
    except Exception as e:
        print(e)
        raise Exception(f"Reading {filename} file encountered an error")

    return data


def normalize_json(data: dict) -> dict:

    all_data = dict()
    for id, data in data.get('data').items():
        new_data = {}
        for key, value in data.items():
            if not isinstance(value, dict):
                new_data[key] = value
            else:
                for k, v in value.items():
                    new_data[k] = v
        all_data[id] = new_data

    return all_data


def generate_csv_data(data: dict) -> str:

    # Defining CSV columns in a list to maintain
    # the order
    csv_columns = data.keys()
    df = pandas.DataFrame.from_dict(data, orient='index')

    return df.to_csv()


def write_to_file(data: str, filepath: str) -> bool:

    try:
        with open(filepath, "w+") as f:
            f.write(data)
    except:
        raise Exception(f"Saving data to {filepath} encountered an error")


def main():
    # Read the JSON file as python dictionary
    data = read_json(filename=str(sys.argv[1]))

    # Normalize the nested python dict
    new_data = normalize_json(data=data)

    # Pretty print the new dict object
    print(new_data)
    # Generate the desired CSV data
    csv_data = generate_csv_data(data=new_data)

    # Save the generated CSV data to a CSV file
    write_to_file(data=csv_data, filepath=sys.argv[2])


if __name__ == '__main__':
    main()
