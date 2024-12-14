// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto::HashValue;
use aptos_framework::generate_next_execution_hash_blob;
use move_core_types::account_address::AccountAddress;
use move_model::{code_writer::CodeWriter, emitln};

pub(crate) fn generate_governance_proposal_header(
    writer: &CodeWriter,
    deps_names: &[&str],
    is_multi_step: bool,
    next_execution_hash: Option<HashValue>,
) {
    emitln!(writer, "script {");
    writer.indent();

    emitln!(writer, "use aptos_framework::aptos_governance;");
    for deps_name in deps_names {
        emitln!(writer, "use {};", deps_name);
    }
    emitln!(writer);

    emitln!(writer, "fun main(proposal_id: u64) {");
    writer.indent();

    if is_multi_step {
        generate_next_execution_hash_blob(writer, AccountAddress::ONE, next_execution_hash);
    } else {
        emitln!(
            writer,
            "let framework_signer = aptos_governance::resolve(proposal_id, @{});\n",
            AccountAddress::ONE,
        );
    }
}

pub(crate) fn generate_testnet_header(writer: &CodeWriter, deps_names: &[&str]) {
    emitln!(writer, "script {");
    writer.indent();

    emitln!(writer, "use aptos_framework::aptos_governance;");
    for deps_name in deps_names {
        emitln!(writer, "use {};", deps_name);
    }
    emitln!(writer);

    emitln!(writer, "fun main(core_resources: &signer) {");
    writer.indent();

    emitln!(
        writer,
        "let core_signer = aptos_governance::get_signer_testnet_only(core_resources, @{});\n",
        AccountAddress::ONE,
    );
    emitln!(writer, "let framework_signer = &core_signer;\n");
}

pub(crate) fn finish_with_footer(writer: &CodeWriter) -> String {
    writer.unindent();
    emitln!(writer, "}");

    writer.unindent();
    emitln!(writer, "}");

    writer.process_result(|s| s.to_string())
}

pub(crate) fn generate_governance_proposal<F>(
    writer: &CodeWriter,
    is_testnet: bool,
    next_execution_hash: Option<HashValue>,
    is_multi_step: bool,
    deps_names: &[&str],
    body: F,
) -> String
where
    F: FnOnce(&CodeWriter),
{
    assert!(
        is_multi_step || next_execution_hash.is_none(),
        "only multi-step proposals can have a next execution hash"
    );

    if is_multi_step {
        generate_governance_proposal_header(writer, deps_names, true, next_execution_hash);
    } else if is_testnet {
        generate_testnet_header(writer, deps_names);
    } else {
        generate_governance_proposal_header(writer, deps_names, false, None);
    }

    body(writer);
    finish_with_footer(writer)
}
