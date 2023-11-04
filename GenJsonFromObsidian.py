import os

# Specify the directory path you want to list files from
directory_path = '../'

# Check if the directory exists
if os.path.exists(directory_path) and os.path.isdir(directory_path):
    # Use os.listdir() to get a list of all files in the directory
    file_names = os.listdir(directory_path)

    # Filter out only the files (not directories)
    file_names = [file for file in file_names if os.path.isfile(os.path.join(directory_path, file)]

    # Print the list of file names
    for file_name in file_names:
        print(file_name)
    else:
        print(f"The directory '{directory_path}' does not exist.")