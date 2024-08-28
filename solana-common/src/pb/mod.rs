pub mod sol {
    pub mod instructions {
        pub mod v1 {
            include!("sol.instructions.v1.rs");
        }
    }
    pub mod transactions {
        pub mod v1 {
            include!("sol.transactions.v1.rs");
        }
    }
}
