#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeKind {
    Stored,
    Pending,
    Permanent,
}

/// A decoded trouble code such as "P0133".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticCode(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CodeStatus {
    #[default]
    NotRequested,
    Requesting,
    Loaded(Vec<DiagnosticCode>), // empty vec = ECU reported no codes
    Failed,
}

/// Lives in memory only, owned by the OBD side.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticReport {
    pub stored: CodeStatus,
    pub pending: CodeStatus,
    pub permanent: CodeStatus,
}

impl DiagnosticReport {
    pub fn set(&mut self, kind: CodeKind, status: CodeStatus) {
        match kind {
            CodeKind::Stored => self.stored = status,
            CodeKind::Pending => self.pending = status,
            CodeKind::Permanent => self.permanent = status,
        }
    }

    /// Call whenever the adapter connects or reinitializes.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
