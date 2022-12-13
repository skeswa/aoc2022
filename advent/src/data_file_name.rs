/// Exposes functionality to generate data file names.
pub trait DataFileNames {
    /// Returns all possible names of a data file given the contents of this
    /// [DataFileName].
    fn to_data_file_names(self) -> Vec<String>;
}

/// Represents a part of a data file name.
pub trait DataFileNameFragment: Copy {
    /// Returns the string fragment that this [DataFileNameFragment] contributes
    /// to a data file name.
    fn to_data_file_name_fragment(self) -> &'static str;
}

impl<T, U> DataFileNames for (&T, &U)
where
    T: DataFileNameFragment,
    U: DataFileNameFragment,
{
    fn to_data_file_names(self) -> Vec<String> {
        vec![
            format!(
                "{}_{}.txt",
                self.0.to_data_file_name_fragment(),
                self.1.to_data_file_name_fragment()
            ),
            format!("{}.txt", self.0.to_data_file_name_fragment(),),
        ]
    }
}
