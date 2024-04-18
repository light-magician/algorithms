impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        /*
        assuming blank is a subsequence of blank and populated string
        edge cases:
            s is blank
            t is blank
            s and t are blank
            s is longer than t
            s has multiple of the same character

        approach:
            make an array of size 26
            get the ascii of each char and put it at that index
        this has to be sequential so this is entirely wrong
        */

        /*
        second approach:
            loop through t, if we find s next each time without
            running out, its all there
        */
        if s.is_empty() {
            return true;
        }
        if t.is_empty() && !s.is_empty() {
            return false;
        }
        let mut si: usize = 0;
        for c in t.chars() {
            if c == s.chars().nth(si).unwrap() {
                si += 1;
            }
            if si == s.len() {
                return true;
            }
        }
        false
    }
}