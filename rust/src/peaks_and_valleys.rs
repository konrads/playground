/// Given:
/// / = step up
/// \ = step down
/// - = stay at same level
/// Assume:
/// peak = start at level 0 (sea level) step up, down (back to level 0)
/// valley = start at level 0 (sea level) step down, up (back to level 0)
///
/// starting at level 0 (sea level) - convert /-\\-/ to 1 peak and 1 valley
fn peaks_and_valleys(s: &str) -> (u32, u32) {
    struct Ctx {
        level: i32,
        peak_cnt: u32,
        valley_cnt: u32
    }

    // let org_ctx = Ctx { level: 0, peak_cnt: 0, valley_cnt: 0 };
    //
    // fn f(ctx: Ctx, c: char) -> Ctx {
    //     match c {
    //         '/' if ctx.level == -1 => Ctx { level: ctx.level+1, valley_cnt: ctx.valley_cnt +1, ..ctx},
    //         '/'                    => Ctx { level: ctx.level+1, ..ctx},
    //         '\\' if ctx.level == 1 => Ctx { level: ctx.level-1, peak_cnt: ctx.peak_cnt +1, ..ctx},
    //         '\\'                   => Ctx { level: ctx.level+1, ..ctx},
    //         _                      => ctx
    //     }
    // }
    //
    // let res = s.chars().fold(org_ctx, f);
    // (res.peak_cnt, res.valley_cnt)

    let res = s.chars().fold(
        Ctx { level: 0, peak_cnt: 0, valley_cnt: 0 },
        |ctx, c|  match c {
            '/' if ctx.level == -1 => Ctx { level: ctx.level+1, valley_cnt: ctx.valley_cnt +1, ..ctx},
            '/'                    => Ctx { level: ctx.level+1, ..ctx},
            '\\' if ctx.level == 1 => Ctx { level: ctx.level-1, peak_cnt: ctx.peak_cnt +1, ..ctx},
            '\\'                   => Ctx { level: ctx.level-1, ..ctx},
            _                      => ctx
        }
    );
    (res.peak_cnt, res.valley_cnt)
}

#[cfg(test)]
mod tests {
    use super::peaks_and_valleys;

    #[test]
    fn peaks_and_valleys_test() {
        assert_eq!(peaks_and_valleys(r"//-\--\\-/"), (1, 1));
    }
}