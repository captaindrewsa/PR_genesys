use crate::parsing::schemas;

use super::db::{workingWithProjects, Project_database};


impl workingWithProjects for Project_database{
    async fn create_project(&mut self, prj_name: &str)-> Result<schemas::databaseQuery,schemas::databaseQuery> {

        
        


        Ok(schemas::databaseQuery::Ok)
    }

    async fn project_is_exist() {
        
    }
}