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
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart { items: vec![], receipt: vec![] }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let index = s.products.iter().position(|(name, _)| ele.cmp(name).is_eq()).unwrap_or(0);
        self.items.push(s.products[index].clone());
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.items.sort_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap());
        
        let item_sale = self.items.len()/3;

        let ratio_promo = 
        &self.items[item_sale..].into_iter().fold(0., |acc, f: &(String, f32)| acc + f.1)/
        &self.items.iter().fold(0., |acc, f| acc + f.1);

        self.receipt = self.items.iter().map(|(_, item_price)| {
            ((item_price * ratio_promo) * 100.).round() / 100.
        }).collect::<Vec<f32>>();

        self.receipt.clone()
    }
}