use utils::terminal_cmds::Command;
use utils::load::terminal_parser::CommandParser;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let parser = CommandParser::new();
    let cmds: Vec<Command> = input.split("\n").into_iter().map(|line| parser.parse(line).unwrap()).collect();

    // Part 1
    let root = make_fs(cmds);
    let sum_smallest = root.sum_smallest();
    println!("/ sum of smallest dirs: {sum_smallest}");

    // Part 2
    const SPACE_AVAIL: u64 = 70_000_000;
    const SPACE_NEEDED: u64 = 30_000_000;
    let unused_space: u64 = SPACE_AVAIL - root.size();
    println!("Space unused before delete: {unused_space}");

    // Try deleting every directory and see how much space it would free up?
    // That would be the size of the directory. So look at every directory's size, and
    // ignore any that won't free up enough space. Then find the smallest.
    let smallest_that_frees_enough: u64 = *all_directory_sizes(&root)
        .iter()
        .filter(|size| unused_space + *size > SPACE_NEEDED)
        .min().unwrap();
    println!("The directory that frees up enough space has size {smallest_that_frees_enough}");
}

fn make_fs(cmds: Vec<Command>) -> Directory {
    let mut iter = cmds.into_iter().skip(1); // First one is the root.
    make_dir(&mut iter)
}

fn make_dir(next_cmds: &mut impl Iterator<Item = Command>) -> Directory {
    let mut current = Directory::new();

    use Command::*;
    while let Some(cmd) = next_cmds.next() {
        match cmd {
            MakeDir => {
                let dir = make_dir(next_cmds);
                current.add_subdir(dir);
            },
            FileSize(file_size) => current.add_file_size(file_size),
            FinishDir => break,
            Ignore => ()
        }
    }

    return current;
}

fn all_directory_sizes(dir: &Directory) -> Vec<u64> {
    let mut sizes = vec![dir.size()];
    sizes.append(
        &mut dir.subdirs
            .iter()
            .map(all_directory_sizes)
            .flatten()
            .collect::<Vec<u64>>()
        );
    return sizes;
}

struct Directory {
    subdirs: Vec<Directory>,
    file_sizes: Vec<u64>
}

impl Directory {
    fn new() -> Directory {
        // If we need to cache the size of a dir, add a field here and invalidate it if we add a file size or subdir.
        Directory { subdirs: vec![], file_sizes: vec![] }
    }

    fn add_file_size(&mut self, size: u64) {
        self.file_sizes.push(size);
    }

    fn add_subdir(&mut self, dir: Directory) {
        self.subdirs.push(dir);
    }

    fn size(&self) -> u64 {
        let dir_size: u64 = self.subdirs.iter().map(|sd| sd.size()).sum();
        dir_size + self.file_sizes.iter().sum::<u64>()
    }

    fn sum_smallest(&self) -> u64 {
        let self_size = self.size();
        let self_size = if self_size <= 100_000 { self_size } else { 0 };
        // Do I need to do sum_smallest *and* size?
        let smallest_subdir_sum: u64 = self.subdirs.iter().map(|s| s.sum_smallest()).sum();
        return self_size + smallest_subdir_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_Directory_test() {
        let mut root = Directory::new();
        root.add_file_size(5);
        let mut sd = Directory::new();
        sd.add_file_size(6);
        sd.add_file_size(7);
        root.add_subdir(sd);
        assert_eq!(root.size(), 18);
    }

    #[test]
    fn sum_smallest() {
        let mut root = Directory::new();
        root.add_file_size(100_001);
        let mut sd = Directory::new();
        sd.add_file_size(99_000);
        let mut sdsd = Directory::new();
        sdsd.add_file_size(500);
        sd.add_subdir(sdsd);
        root.add_subdir(sd);
        assert_eq!(root.size(), 199_501);
        assert_eq!(root.sum_smallest(), 100_000); // sdsd gets double-counted!
    }
}