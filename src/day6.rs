

// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let str1_trm = s1.trim();
    let str2_trm = s2.trim();

    //type converting to i32 because value could be negative
    let result: i32 = str1_trm.chars().count() as i32 - str2_trm.chars().count() as i32;

    if result > 0 {
        return Some(str1_trm) //returning trimmed reference
    }else if result < 0 {
        return Some(str2_trm)
    } else {
        return None
    }
}
