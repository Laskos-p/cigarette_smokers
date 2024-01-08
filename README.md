# Cigarette smokers problem in Rust

Solution to cigarette smokers problem in Rust using Mutexes and message passing with channels.

This cigarette smokers problem has extra constraints:
- agent sets the prices of ingredients
- every smoker have one ingredient which he sells to other smokers
- smoker must buy two ingredients at once
- smokers have some money to begin with
- if smoker has no money to buy ingredients he must wait for price change or money from sold ingredients
- smokers use message passing to transfer money for ingredients
