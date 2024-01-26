mod filesystem;

use filesystem::directory::Directory;
use filesystem::file::File;

fn main() {
    // create directory
    let mut root_directory = Directory::new("Root");

    // create file
    let file1 = File::new("file1.txt", "Content of file1");
    let file2 = File::new("file2.txt", "Content of file2");

    // add file to directory
    root_directory.add_file(file1);
    root_directory.add_file(file2);

    // list of all files in directory
    root_directory.list_files();

    // file read
    let file3 = File::new("file3.txt", "Content of file3");
    file3.read();
}
