// Neste arquivo são criadas as regras de negócios e as estruturas de dados, entidades, agregados, value objects e tipos, podendo ser dividida em submódulos para cada propósito

pub mod types {
use chrono::prelude::{DateTime, Utc};
  type Id = String; 
  type DataHora = DateTime<Utc>;
}

pub mod entity {
 //Eu deixei a entidade para poder ser acessada pela layer models::Todo porém em geral as entidades ficam neste pacote sendo expostos apenas os seus agregados
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<Id>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<DataHora>,
    pub updated_at: Option<DataHora>,
}


