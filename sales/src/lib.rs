#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub products: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            products: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((_, price)) = s.products.iter().find(|(name, _)| *name == ele) {
            self.products.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.products.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if prices.len() < 3 {
            self.receipt = prices;
            return self.receipt.clone();
        }

        let discount_total: f32 = prices.iter().take(prices.len() / 3).sum();
        let original_total: f32 = prices.iter().sum();
        let discount_ratio: f32 = discount_total / original_total;

        self.receipt = prices
            .iter()
            .map(|price| ((price * (1.0 - discount_ratio) * 100.0).round()) / 100.0)
            .collect();

        self.receipt.clone()
    }
}
