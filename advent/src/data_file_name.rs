/// Name of a data file.
pub trait DataFileName {
    /// Represents this object as a data file name [String].
    fn to_data_file_name(self) -> String;
}

/// Represents a part of a data file name.
pub trait DataFileNameFragment: Copy {
    /// Returns the string fragment that this [DataFileNameFragment] contributes
    /// to a data file name.
    fn to_data_file_name_fragment(self) -> &'static str;
}

impl<T, U> DataFileName for (&T, &U)
where
    T: DataFileNameFragment,
    U: DataFileNameFragment,
{
    fn to_data_file_name(self) -> String {
        format!(
            "{}_{}.txt",
            self.0.to_data_file_name_fragment(),
            self.1.to_data_file_name_fragment()
        )
    }
}
