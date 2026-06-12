use pyo3::prelude::*;

#[derive(Clone)]
pub enum SyntheticGraphType {
    ErdosRenyi { n: usize, p: f64 },
    BarabasiAlbert { n: usize, m: usize },
    WattsStrogatz { n: usize, k: usize, beta: f64 },
}

#[pyclass(name = "SyntheticGraphType", module = "netfog")]
#[derive(Clone)]
pub struct PySyntheticGraphType {
    pub inner: SyntheticGraphType,
}

#[pymethods]
impl PySyntheticGraphType {
    #[staticmethod]
    pub fn erdos_renyi(n: usize, p: f64) -> Self {
        PySyntheticGraphType { inner: SyntheticGraphType::ErdosRenyi { n, p } }
    }

    #[staticmethod]
    pub fn barabasi_albert(n: usize, m: usize) -> Self {
        PySyntheticGraphType { inner: SyntheticGraphType::BarabasiAlbert { n, m } }
    }

    #[staticmethod]
    pub fn watts_strogatz(n: usize, k: usize, beta: f64) -> Self {
        PySyntheticGraphType { inner: SyntheticGraphType::WattsStrogatz { n, k, beta } }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            SyntheticGraphType::ErdosRenyi { n, p } => format!("SyntheticGraphType.erdos_renyi(n={}, p={})", n, p),
            SyntheticGraphType::BarabasiAlbert { n, m } => format!("SyntheticGraphType.barabasi_albert(n={}, m={})", n, m),
            SyntheticGraphType::WattsStrogatz { n, k, beta } => format!("SyntheticGraphType.watts_strogatz(n={}, k={}, beta={})", n, k, beta),
        }
    }
}
