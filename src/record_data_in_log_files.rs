use std::cmp::Ordering;

/// 区分日志
pub fn divide_logs(logs: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut letter_log = Vec::new();
    let mut number_log = Vec::new();
    for log in logs {
        let l = log.splitn(2, ' ').collect::<Vec<&str>>();
        let is_number = |s: &str| -> bool { s.chars().next().unwrap().is_numeric() };
        if is_number(l[1]) {
            number_log.push(log);
        } else {
            letter_log.push(log);
        }
    }
    (number_log, letter_log)
}

fn tag_enumerator(log: &str) -> usize {
    log.find(' ').unwrap_or_default()
}

fn sort_rule(log1: &str, log2: &str) -> Ordering {
    let log1_tag = tag_enumerator(log1);
    let log2_tag = tag_enumerator(log2);
    if log1[log1_tag..].cmp(&log2[log2_tag..]) == Ordering::Equal {
        log1[..log1_tag].cmp(&log2[..log2_tag])
    } else {
        log1[log1_tag..].cmp(&log2[log2_tag..])
    }
}

pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
    let (number_log, mut letter_log) = divide_logs(logs);
    letter_log.sort_by(|a, b| sort_rule(a, b));
    letter_log.extend(number_log);
    letter_log
}
