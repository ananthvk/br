# Bulk renamer

## Goal
Build a CLI application which can bulk rename multiple files/folders based on a specified pattern, file extension, or some other file parameter. 

## Required features
- Selecting files (filters) based on
    1. Presence of a substring
    2. Start or end with with a particular string
    3. A range of creation/modification/access time
    4. Presence of a date in the specified format in the filename
    5. File extension
    6. File attributes - hidden, readonly, etc
    7. Filename length
    8. Advanced - Regex

- Operations
    1. Replace substring
    2. Replace regex
    3. Modify extension
    4. Prefix and suffix string
    5. Remove the first or last n characters
    6. Move the file to a different folder, (depending upon the filename, nested folders)
    7. Automatic numbering, along with option to pad zeroes
    8. Change the format of date in the file
    9. Convert date -> serial number
    10. Append creation time, modification time, etc to filename
    11. Remove spaces

## Extras

There should also be an option to specify ignore filters

Both filters and operations should be combinable, i.e. multiple filters and options can be specified

An option to dry run (does not carry out the operation, just displays it)

Recursive flag, and max recursion depth

Log file to view the operations performed

## Tasks list
1. [x] Implement basic command line handling
2. [x] Next implement regex substitution
3. Add the recursive and dry run flags
4. TODO
