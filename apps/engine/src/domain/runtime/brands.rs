const GREASEY_CHARS: [&str; 11] = [" ", "(", ":", "-", ".", "/", ")", ";", "=", "?", "_"];
const GREASED_VERSIONS: [&str; 3] = ["8", "99", "24"];
const ORDERS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrandVersion {
    pub brand: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChromeBrands {
    pub major: Vec<BrandVersion>,
    pub full: Vec<BrandVersion>,
}

pub fn chrome_brands(full_version: &str) -> ChromeBrands {
    let major_str = full_version.split('.').next().unwrap_or(full_version);
    let seed = major_str.parse::<usize>().unwrap_or(0);
    let pick = |items: &[&'static str], i: usize| items.get(i % items.len()).copied().unwrap_or("");
    let grease_brand = format!(
        "Not{}A{}Brand",
        pick(&GREASEY_CHARS, seed),
        pick(&GREASEY_CHARS, seed + 1)
    );
    let grease_major = pick(&GREASED_VERSIONS, seed);
    let build = |grease_ver: String, ver: &str| {
        shuffle(
            [
                bv(&grease_brand, &grease_ver),
                bv("Chromium", ver),
                bv("Google Chrome", ver),
            ],
            seed,
        )
    };
    ChromeBrands {
        major: build(grease_major.to_owned(), major_str),
        full: build(format!("{grease_major}.0.0.0"), full_version),
    }
}

fn bv(brand: &str, version: &str) -> BrandVersion {
    BrandVersion {
        brand: brand.to_owned(),
        version: version.to_owned(),
    }
}

fn shuffle(list: [BrandVersion; 3], seed: usize) -> Vec<BrandVersion> {
    let order = ORDERS
        .get(seed % ORDERS.len())
        .copied()
        .unwrap_or([0, 1, 2]);
    let mut slots: [Option<BrandVersion>; 3] = [None, None, None];
    for (item, &pos) in list.into_iter().zip(order.iter()) {
        if let Some(slot) = slots.get_mut(pos) {
            *slot = Some(item);
        }
    }
    slots.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::chrome_brands;

    fn names(v: &[super::BrandVersion]) -> Vec<String> {
        v.iter()
            .map(|b| format!("{}={}", b.brand, b.version))
            .collect()
    }

    #[test]
    fn matches_native_chrome_135() {
        let b = chrome_brands("135.0.7049.84");
        assert_eq!(
            names(&b.major),
            ["Google Chrome=135", "Not-A.Brand=8", "Chromium=135"]
        );
        assert_eq!(
            names(&b.full),
            [
                "Google Chrome=135.0.7049.84",
                "Not-A.Brand=8.0.0.0",
                "Chromium=135.0.7049.84"
            ]
        );
    }

    #[test]
    fn matches_native_chrome_151() {
        let b = chrome_brands("151.0.7922.108");
        assert_eq!(
            names(&b.major),
            ["Not=A?Brand=99", "Google Chrome=151", "Chromium=151"]
        );
    }

    #[test]
    fn matches_native_chrome_154() {
        let b = chrome_brands("154.0.8037.57");
        assert_eq!(
            names(&b.major),
            ["Chromium=154", "Google Chrome=154", "Not A(Brand=99"]
        );
    }
}
