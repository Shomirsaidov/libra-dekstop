import Database from "tauri-plugin-sql-api";

class DatabaseService {
  constructor() {
    // Initialize the database connection here if needed
  }

  async connectToDatabase() {
    try {
      // sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
      return await Database.load("sqlite:./mydata.db");
      // You can also connect to mysql sor postgres as needed
    } catch (error) {
      console.error("Error connecting to database:", error);
      throw error;
    }
  }

  async executeQuery(query, params) {
    try {
      const db = await this.connectToDatabase();
      return await db.execute(query, params);
    } catch (error) {
      console.error("Error executing query:", error);
      throw error;
    }
  }
}

export default new DatabaseService();
