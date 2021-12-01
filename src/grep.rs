#[derive(Debug, PartialEq)]
struct GrepResult {
    number: u32,
    line: String,
}

impl GrepResult {
    fn new(number:u32, line:String) -> GrepResult {
        GrepResult{number, line}
    }
}

fn grep(word: &str, lines:Vec<String>) -> Vec<GrepResult>{
    let mut results: Vec<GrepResult> = Vec::new();
    for (number, line) in lines.iter().enumerate() {
        if line.contains(word){
            results.push(GrepResult::new((number+1).try_into().unwrap(), line.to_string()))
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::io::BufRead;

    fn file_as_lines(path:&str) -> Result<Vec<String>>{
        let file = std::fs::File::open(path)?;
        let mut lines: Vec<String> = Vec::new();
        for line in std::io::BufReader::new(file).lines(){
            lines.push(line?);
        }
        Ok(lines)
    }

    #[test]
    fn test_grep(){
        let lines = file_as_lines("test/lorem.txt").unwrap();
        let want = vec![super::GrepResult::new(2, "Maecenas mollis nunc velit, at pulvinar nibh cursus at. Nam id erat vestibulum, varius sem in, ultricies quam. Nulla porttitor mattis convallis. Proin nec lorem ante. Integer varius blandit est, quis scelerisque elit euismod quis. Aliquam erat volutpat. Fusce ac dignissim ante, suscipit aliquam risus. Morbi lacinia, nunc non pretium egestas, libero dui fermentum elit, vitae vulputate urna purus ac ipsum. Nunc eget porta arcu. In vel massa sollicitudin, imperdiet arcu id, mattis massa. Ut porta mauris vulputate ipsum ullamcorper bibendum. Maecenas ut euismod sapien, nec tempus justo. Nunc a aliquet ex. In hac habitasse platea dictumst.".to_string())];
        let got = super::grep("pulvinar", lines);
        assert_eq!(want, got);
    }
}