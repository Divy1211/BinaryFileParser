use std::cmp::max;
use crate::types::diff::{Diff, Diffable};

use itertools::{Itertools, EitherOrBoth::*};

impl<T> Diffable<T> for Vec<T>
where
    T: Eq + Clone + Diffable<T>,
{
    fn diff(&self, other: &Self) -> Diff<T> {
        let ls_lcs = lcs(&self, &other, 0, 0);
            
        let (mut start1, mut start2) = (0, 0);
        let mut diff = vec![];
        
        for (_item, idx1, idx2) in ls_lcs {
            for values in self[start1..idx1].iter().enumerate().zip_longest(other[start2..idx2].iter()) {
                match values {
                    Right(right) => {
                        diff.push((idx1, Diff::Inserted(right.clone())))
                    }
                    Left((i, left)) => {
                        diff.push((i, Diff::Deleted(left.clone())))
                    }
                    Both((i, left), right) => {
                        diff.push((i, left.diff(right)))
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
                    diff.push((i, Diff::Deleted(left.clone())))
                }
                Both((i, left), right) => {
                    diff.push((i, left.diff(right)))
                }
            }
        }

        if diff.len() == 0 {
            return Diff::None;
        }
        
        Diff::Nested(diff)
    }
}

fn idx(i: isize, size: usize) -> usize {
    (if i < 0 {
        i + size as isize
    } else {
        i
    }) as usize
}

fn lcs<'c, 'a: 'c, 'b: 'c, T: Eq>(a: &'a [T], b: &'b [T], i: usize, j: usize) -> Vec<(&'c T, usize, usize)> {
    let n = a.len();
    let m = b.len();
    let size = n + m;
    
    if n <= 0 || m <= 0 {
        return vec![];
    }
    
    let delta = n as isize - m as isize;
    let mut vf = vec![0_usize; size + 2];
    let mut vb = vec![0_usize; size + 2];
    for d in 0..=((size+1)/2) as isize {
        // forward
        for k in (-(d - 2*max(0, d - m as isize))..=(d - 2*max(0, d - n as isize))).step_by(2) {
            let km1 = idx(k-1, size);
            let kp1 = idx(k+1, size);
            let ku = idx(k, size);
            let mut x = if k == -d || k != d && vf[km1] < vf[kp1] {
                vf[kp1]
            } else {
                vf[km1]+1
            };
            let mut y = (x as isize - k) as usize;
            let (sx, sy) = (x, y);
            while x < n && y < m && a[x] == b[y] {
                (x, y) = (x + 1, y + 1);
            }
            vf[ku] = x;
            let z = -(k-delta);
            if size % 2 == 1 && -(d-1) <= z && z <= (d-1) && vf[ku] + vb[idx(z, size)] >= n {
                let d = 2*d-1;
                return if d > 1 || sx != x {
                    let mut r = Vec::with_capacity(size);
                    r.extend(lcs(&a[0..sx], &b[0..sy], i, j));
                    r.extend(a[sx..x].iter().enumerate().map(|(l, e)| (e, i + sx + l, j + sy + l)));
                    r.extend(lcs(&a[x..n], &b[y..m], i + x, j + y));
                    r
                } else if m > n {
                    a[0..n].iter().enumerate().map(|(l, e)| (e, i+l, j+l)).collect()
                } else {
                    b[0..m].iter().enumerate().map(|(l, e)| (e, i+l, j+l)).collect()
                }
            }
        }

        // backward
        for k in -(d - 2*max(0, d - m as isize))..=(d - 2*max(0, d - n as isize)) {
            let km1 = idx(k-1, size);
            let kp1 = idx(k+1, size);
            let ku = idx(k, size);
            let mut x = if k == -d || k != d && vb[km1] < vb[kp1] {
                vb[kp1]
            } else {
                vb[km1]+1
            };
            let mut y = (x as isize - k) as usize;
            let (sx, sy) = (x, y);
            while x < n && y < m && a[n-x-1] == b[m-y-1] {
                (x, y) = (x + 1, y + 1);
            }
            vb[ku] = x;
            let z = -(k-delta);
            if size % 2 == 0 && -d <= z && z <= d && vb[ku] + vf[idx(z, size)] >= n {
                let d = 2*d;
                return if d > 1 || sx != x {
                    let mut r = Vec::with_capacity(size);
                    r.extend(lcs(&a[0..n-x], &b[0..m-y], i, j));
                    r.extend(a[n-x..n-sx].iter().enumerate().map(|(l, e)| (e, i + n-x + l, j + m-y + l)));
                    r.extend(lcs(&a[n-sx..n], &b[m-sy..m], i + n-sx, j + m-sy));
                    r
                } else if m > n {
                    a[0..n].iter().enumerate().map(|(l, e)| (e, i+l, j+l)).collect()
                } else {
                    b[0..m].iter().enumerate().map(|(l, e)| (e, i+l, j+l)).collect()
                }
            }
        }
    }
    
    vec![]
}
