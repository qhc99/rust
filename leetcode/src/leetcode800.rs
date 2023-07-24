#[allow(dead_code)]
/// #813
pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();

    let mut prefix_sum = vec![0];
    prefix_sum.reserve(n);
    prefix_sum.extend(nums);
    for i in 1..prefix_sum.len() {
        prefix_sum[i] += prefix_sum[i - 1];
    }

    let mut dp: Vec<f64> = Vec::new();
    dp.reserve(prefix_sum.len());
    prefix_sum.iter().for_each(|v| -> () {
        dp.push(v.clone() as f64);
    });

    for i in 1..dp.len() {
        dp[i] /= i as f64;
    }

    for j in 2..=k {
        for i in (1..=n).rev() {
            for x in (j - 1) as usize..=(i - 1) {
                dp[i] = f64::max(
                    dp[i],
                    dp[x] + (prefix_sum[i] - prefix_sum[x]) as f64 / (i - x) as f64,
                );
            }
        }
    }

    return dp[n];
}

#[allow(dead_code)]
/// #815
pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    use std::collections::{HashMap, HashSet, VecDeque};
    if source == target {
        return 0;
    }
    let cap = routes.iter().map(|v| -> usize { v.len() }).sum();
    let mut bus_stop2_routes: HashMap<i32, Vec<i32>> = HashMap::new();
    bus_stop2_routes.reserve(cap);
    for (route_idx, route_stops) in routes.iter().enumerate() {
        for stop in route_stops.iter() {
            let route_idx = route_idx as i32;
            if !bus_stop2_routes.contains_key(stop) {
                bus_stop2_routes.insert(stop.clone(), vec![route_idx]);
            } else {
                bus_stop2_routes.get_mut(stop).unwrap().push(route_idx);
            }
        }
    }
    let mut queue_routes: VecDeque<(i32, i32)> = VecDeque::new();

    let start_routes = bus_stop2_routes.get(&source);
    let end_routes = bus_stop2_routes.get(&target);
    if start_routes.is_none() || end_routes.is_none() {
        return -1;
    }
    let start_routes = start_routes.unwrap();
    let end_routes = end_routes.unwrap();
    start_routes
        .iter()
        .for_each(|v| -> () { queue_routes.push_back((v.clone(), 1)) });

    let mut end_routes_set: HashSet<i32> = HashSet::new();
    end_routes_set.reserve(end_routes.len());
    for i in end_routes {
        end_routes_set.insert(i.clone());
    }
    let mut visited_routes = HashSet::new();
    while !queue_routes.is_empty() {
        let current_route = queue_routes.pop_front().unwrap();
        let count = current_route.1;
        let current_route = current_route.0;
        if end_routes_set.contains(&current_route) {
            return count;
        }
        let current_stops = &routes[current_route as usize];
        for stop in current_stops {
            let next_routes = bus_stop2_routes.get(stop);
            if next_routes.is_some() {
                next_routes.unwrap().iter().for_each(|r| -> () {
                    if !visited_routes.contains(r) {
                        queue_routes.push_back((r.clone(), count + 1));
                        visited_routes.insert(r.clone());
                    }
                });
            }
        }
    }
    return -1;
}

/// #816
pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    fn valid_comma_split(s: &[u8]) -> bool {
        if s.len() == 1 {
            return true;
        } else {
            return s[0] != b'0' || s[s.len() - 1] != b'0';
        }
    }

    fn dot_split(s1: &[u8], s2: &[u8], v: &mut Vec<String>) {
        let s1 = all_dot_split(s1);
        let s2 = all_dot_split(s2);
        for left in s1.iter() {
            for right in s2.iter() {
                let mut t = vec![b'('];
                t.append(&mut left.as_bytes().to_vec());
                t.push(b',');
                t.push(b' ');
                t.append(&mut right.as_bytes().to_vec());
                t.push(b')');
                v.push(String::from_utf8(t).unwrap());
            }
        }
    }

    fn all_dot_split(s: &[u8]) -> Vec<String> {
        use std::str;
        let mut ans = vec![];
        if s.len() == 1 {
            ans.push(str::from_utf8(s).unwrap().to_string());
            return ans;
        }

        if s[0] == b'0' {
            // only 0.###
            let mut t = vec![b'0', b'.'];
            t.append(&mut (s[1..].to_vec()));
            ans.push(String::from_utf8(t).unwrap());
            return ans;
        } else if s[s.len() - 1] == b'0' {
        } else {
            // can split without prefix and suffix 0
            for r_start in 1..s.len() {
                let mut t = s[0..r_start].to_vec();
                t.push(b'.');
                t.append(&mut (s[r_start..].to_vec()));
                ans.push(String::from_utf8(t).unwrap());
            }
        }
        // add self if no prefix 0
        ans.push(str::from_utf8(s).unwrap().to_string());
        return ans;
    }

    let chrs = s.as_bytes();
    let end = s.len() - 1;
    let mut ans = vec![];
    for right_start in 2..end {
        let s1 = &chrs[1..right_start];
        let s2 = &chrs[right_start..end];
        if valid_comma_split(s1) && valid_comma_split(s2) {
            dot_split(s1, s2, &mut ans)
        }
    }

    return ans;
}
