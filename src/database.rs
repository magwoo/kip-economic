use anyhow::Result;
use sled::Db;

use crate::Employee;

#[derive(Clone)]
pub struct Database(Db);

impl Database {
    pub fn add(&self, employee: Employee) -> Result<()> {
        let tree = self.0.open_tree("employees")?;
        let data = serde_json::to_vec(&employee)?;

        tree.insert(self.increment()?.to_be_bytes(), data)?;

        Ok(())
    }

    pub fn remove(&self, id: u32) -> Result<Option<Employee>> {
        let tree = self.0.open_tree("employees")?;

        Ok(tree
            .remove(id.to_be_bytes())
            .map(|o| o.map(|e| serde_json::from_slice(&e).unwrap()))?)
    }

    pub fn checksum(&self) -> Result<u32> {
        Ok(self.0.checksum()?)
    }

    pub fn get_all(&self) -> Result<Vec<(u32, Employee)>> {
        let tree = self.0.open_tree("employees")?;
        let mut employees = Vec::new();

        for data in tree.into_iter() {
            let data = data?;
            let id = u32::from_be_bytes(data.0[0..4].try_into()?);
            let employee = serde_json::from_slice(&data.1)?;
            employees.push((id, employee));
        }

        Ok(employees)
    }

    fn increment(&self) -> Result<u32> {
        let tree = self.0.open_tree("counter")?;

        let counter = tree
            .get("counter")?
            .map(|iv| iv.to_vec())
            .unwrap_or([0, 0, 0, 0].to_vec());

        let counter = u32::from_be_bytes(counter[0..4].try_into()?);

        tree.insert("counter", &(counter + 1).to_be_bytes())?;

        Ok(counter)
    }
}

impl Default for Database {
    fn default() -> Self {
        let db = sled::open("./data").unwrap();
        Self(db)
    }
}
