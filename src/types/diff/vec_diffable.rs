use std::cmp::min;
use std::fmt::Debug;

use itertools::{EitherOrBoth::*, Itertools};

use crate::types::diff::diff::{Diff, Diffable};

impl<T> Diffable<T> for Vec<T>
where
    T: Eq + Clone + Debug + Diffable<T>,
{
    fn diff(&self, other: &Self) -> Diff<T> {
        let ls_lcs = lcs(self, other);

        let (mut start1, mut start2) = (0, 0);
        let mut diff = vec![];

        for (_item, idx1, idx2) in ls_lcs {
            for values in self[start1..idx1].iter().enumerate().zip_longest(other[start2..idx2].iter()) {
                match values {
                    Right(right) => {
                        diff.push((idx1, Diff::Inserted(right.clone())))
                    }
                    Left((i, left)) => {
                        diff.push((i + start1, Diff::Deleted(left.clone())))
                    }
                    Both((i, left), right) => {
                        diff.push((i + start1, left.diff(right)))
                    }
                }
            }
            (start1, start2) = (idx1 + 1, idx2 + 1);
        }
        let (idx1, idx2) = (self.len(), other.len());
        for values in self[start1..idx1].iter().enumerate().zip_longest(other[start2..idx2].iter()) {
            match values {
                Right(right) => {
                    diff.push((idx1, Diff::Inserted(right.clone())))
                }
                Left((i, left)) => {
                    diff.push((i + start1, Diff::Deleted(left.clone())))
                }
                Both((i, left), right) => {
                    diff.push((i + start1, left.diff(right)))
                }
            }
        }

        if diff.is_empty() {
            return Diff::None;
        }

        Diff::Nested(diff)
    }
}

fn lcs<'c, 'a: 'c, 'b: 'c, T: Eq + Debug>(a: &'a [T], b: &'b [T]) -> Vec<(&'c T, usize, usize)> {
    let mut result = vec![];
    lcs_rec(a, b, 0, 0, true, &mut result);
    result
}

fn lcs_rec<'c, 'a: 'c, 'b: 'c, T: Eq + Debug>(a: &'a [T], b: &'b [T], i: usize, j: usize, first: bool, result: &mut Vec<(&'c T, usize, usize)>) {
    let n = a.len();
    let m = b.len();
    let size = n + m;

    if n == 0 || m == 0 {
        return;
    }

    if n == 1 {
        for (k, e) in b.iter().enumerate() {
            if a[0] == *e {
                result.push((e, i, j + k));
                return;
            }
        }
        return;
    }

    if m == 1 {
        for (k, e) in a.iter().enumerate() {
            if b[0] == *e {
                result.push((e, i + k, j));
                return;
            }
        }
        return;
    }

    if first {
        let mut start_f = -1_isize;
        let mut start_b = -1_isize;
        for k in 0..min(n, m) {
            if a[k] != b[k] && start_f == -1 {
                start_f = k as isize;
            }
            if start_f != -1 {
                break;
            }
        }
        let start_f = if start_f == -1 {
            min(n, m)
        } else {
            start_f as usize
        };

        for k in 0..min(n - start_f, m - start_f) {
            if a[n - k - 1] != b[m - k - 1] && start_b == -1 {
                start_b = k as isize;
            }
            if start_b != -1 {
                break;
            }
        }
        let start_b = if start_b == -1 {
            min(n - start_f, m - start_f)
        } else {
            start_b as usize
        };

        for (k, v) in a.iter().enumerate().take(start_f) {
            result.push((v, k, k));
        }
        lcs_rec(&a[start_f..n-start_b], &b[start_f..m-start_b], start_f, start_f, false, result);
        for k in (0..start_b).rev() {
            result.push((&a[n-k-1], n-k-1, m-k-1));
        }
        return;
    }

    let mut vf = vec![0_usize; size + 3];
    let mut vb = vec![n; size + 3];
    let delta = n as isize - m as isize;

    let s2 = ((size+1)/2) as isize;

    for d in 0..=s2 {
        for k in (-d..=d).step_by(2) {
            let ki = (k + s2 + 1) as usize;
            let mut x = if k == -d || k != d && vf[ki-1] < vf[ki+1] {
                vf[ki+1]
            } else {
                if vf[ki-1]+1 > n {
                    continue;
                }
                vf[ki-1]+1
            };
            if k > x as isize {
                continue;
            }

            let mut y = (x as isize - k) as usize;
            let (sx, sy) = (x, y);
            while x < n && y < m && a[x] == b[y] {
                (x, y) = (x+1, y+1);
            }
            vf[ki] = x;
            let kd = ki as isize - delta;

            if delta%2 != 0
                && delta-d < k && k < delta+d
                && 0 <= kd && kd < (size+3) as isize
                && vf[ki] >= vb[kd as usize] {

                let d = 2*d-1;
                if d > 1 {
                    lcs_rec(&a[..sx], &b[..sy], i, j, false, result);
                    result.extend(a[sx..x].iter().enumerate().map(|(k, e)| (e, k + i + sx, k + j + sy)));
                    lcs_rec(&a[x..], &b[y..], i + x, j + y, false, result);
                    return;
                }
            }
        }

        for k in (-d..=d).step_by(2) {
            let ki = (k + s2 + 1) as usize;
            let mut x = if k == d || k != -d && vb[ki-1] < vb[ki+1] {
                vb[ki-1]
            } else {
                if vb[ki+1] == 0 {
                    continue;
                }
                vb[ki+1]-1
            };
            if k + delta > x as isize {
                continue;
            }
            let mut y = (x as isize - k - delta) as usize;
            let (sx, sy) = (x, y);
            while x > 0 && y > 0 && x <= n && y <= m && a[x-1] == b[y-1] {
                (x, y) = (x-1, y-1);
            }
            vb[ki] = x;

            let (sx, sy, x, y) = (x, y, sx, sy);

            let kd = ki as isize + delta;

            if delta%2 == 0
                && -delta-d <= k && k <= -delta+d
                && 0 <= kd && kd < (size+3) as isize
                && vf[kd as usize] >= vb[ki] {
                let d = 2*d;
                if d > 1 {
                    lcs_rec(&a[..sx], &b[..sy], i, j, false, result);
                    result.extend(a[sx..x].iter().enumerate().map(|(k, e)| (e, k + i + sx, k + j + sy)));
                    lcs_rec(&a[x..], &b[y..], i + x, j + y, false, result);
                    return;
                }
            }
        }
    }

    unreachable!("Internal BFP error occured");
}