use aoc_utils::AocError;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut disk = parse(input)?;
    compact(&mut disk);
    let checksum = checksum(&disk);
    Ok(checksum)
}

fn compact(disk: &mut Disk) {
    let mut head = 0;
    let mut tail = disk.len();

    while head < tail {
        while head < tail && disk.blocks[head].is_file() {
            head += 1;
        }

        while head < tail && disk.blocks[head].is_free() {
            loop {
                tail -= 1;
                if disk.blocks[tail].is_file() {
                    break;
                }
            }

            disk.blocks.swap(head, tail);
            head += 1;
        }
    }

    let span = Span::new(head as u64, (disk.len() - head) as u64);
    disk.free_spans.clear();
    disk.free_spans.push(span);
}

fn checksum(disk: &Disk) -> u64 {
    disk.blocks
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::File(id) => *id * index as u64,
            Block::Free => 0,
        })
        .sum()
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut disk = parse(input)?;
    defragment(&mut disk);
    let checksum = checksum(&disk);
    Ok(checksum)
}

fn defragment(disk: &mut Disk) {
    let mut file_index = disk.files.len().checked_sub(1).unwrap_or(0);
    while file_index > 0 {
        defragment_file(disk, file_index);
        file_index -= 1;
    }
}

fn defragment_file(disk: &mut Disk, file_index: usize) -> Option<()> {
    let file = *disk.files.get(file_index)?;

    let free_span_index = disk
        .free_spans
        .iter()
        .position(|span| span.len() >= file.len())?;

    let free_span = disk.free_spans.get_mut(free_span_index)?;
    if free_span.start() >= file.start() {
        return None;
    }

    for index in file.start()..file.end() {
        disk.blocks.swap(free_span.start as usize, index as usize);
        free_span.start += 1;
    }

    if free_span.is_empty() {
        disk.free_spans.remove(free_span_index);
    }

    Some(())
}

fn parse(input: &str) -> anyhow::Result<Disk> {
    let mut disk = Disk::default();
    for (index, c) in input.trim().chars().enumerate() {
        let is_file = index % 2 == 0;
        let len = c.to_digit(10).ok_or(AocError::InvalidInput)? as u64;

        if len == 0 {
            debug_assert!(!is_file);
            continue;
        }

        if is_file {
            disk.push_file(len);
        } else {
            disk.push_free(len);
        }
    }

    Ok(disk)
}

#[derive(Debug, Default, Clone)]
struct Disk {
    blocks: Vec<Block>,
    files: Vec<File>,
    free_spans: Vec<Span>,
}

impl Disk {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn push_file(&mut self, len: u64) {
        let file_id = self.files.len() as u64;
        let start = self.blocks.len() as u64;
        for _ in 0..len {
            self.blocks.push(Block::File(file_id));
        }

        let span = Span::new(start, len);
        let file = File::new(file_id, span);
        self.files.push(file);
    }

    pub fn push_free(&mut self, len: u64) {
        let start = self.blocks.len() as u64;
        for _ in 0..len {
            self.blocks.push(Block::Free);
        }

        let span = Span::new(start, len);
        self.free_spans.push(span);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    File(u64),
    Free,
}

impl Block {
    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    pub const fn is_free(&self) -> bool {
        matches!(self, Self::Free { .. })
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(id) => write!(f, "{id}"),
            Self::Free => write!(f, "_"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct File {
    id: u64,
    span: Span,
}

impl File {
    pub const fn new(id: u64, span: Span) -> Self {
        Self { id, span }
    }
}

impl std::ops::Deref for File {
    type Target = Span;

    fn deref(&self) -> &Self::Target {
        &self.span
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Span {
    start: u64,
    end: u64,
}

impl Span {
    pub const fn new(start: u64, len: u64) -> Self {
        Self {
            start,
            end: start + len,
        }
    }

    pub const fn len(&self) -> u64 {
        self.end - self.start
    }

    pub const fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    pub const fn start(&self) -> u64 {
        self.start
    }

    pub const fn end(&self) -> u64 {
        self.end
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");

    #[rstest]
    #[case(EXAMPLE_1, 1928)]
    #[case(EXAMPLE_2, 62)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        let result = part_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(EXAMPLE_1, 2858)]
    #[case(EXAMPLE_2, 132)]
    fn test_part_2(#[case] input: &str, #[case] expected: u64) {
        let result = part_2(input).unwrap();
        assert_eq!(result, expected)
    }
}
