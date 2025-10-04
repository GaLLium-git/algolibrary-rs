pub fn angle_sort(points: &mut Vec<(i64, i64)>) {
    let half = |(x, y): &(i64, i64)| -> u8 {
        if *y > 0 || (*y == 0 && *x >= 0) { 0 } else { 1 }
    };

    points.sort_by(|a, b| {
        let ha = half(a);
        let hb = half(b);
        if ha != hb {
            return ha.cmp(&hb);
        }

        let ax = a.0 as i128;
        let ay = a.1 as i128;
        let bx = b.0 as i128;
        let by = b.1 as i128;
        let cross = ax * by - ay * bx;
        if cross > 0 {
            Ordering::Less 
        } else if cross < 0 {
            Ordering::Greater
        } else {
            let da = ax * ax + ay * ay;
            let db = bx * bx + by * by;
            da.cmp(&db)
        }
    });
}
