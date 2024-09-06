use rustc_middle::mir::coverage::CoverageKind;
use rustc_middle::mir::SourceScope;

use super::FunctionCx;
use crate::traits::*;

impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>> FunctionCx<'a, '_, 'tcx, Bx> {
    pub fn codegen_coverage(&self, bx: &mut Bx, kind: &CoverageKind, scope: SourceScope) {
        // Determine the instance that coverage data was originally generated for.
        let instance = scope.inlined_instance(&self.mir.source_scopes).unwrap_or(self.instance);

        // Handle the coverage info in a backend-specific way.
        bx.add_coverage(instance, kind);
    }
}
