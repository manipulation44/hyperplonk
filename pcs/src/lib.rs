mod errors;
mod multilinear_kzg;
mod structs;
mod univariate_kzg;

pub mod prelude;

use ark_ec::PairingEngine;
use ark_std::rand::RngCore;
use errors::PCSErrors;

/// This trait defines APIs for polynomial commitment schemes.
/// Note that for our usage of PCS, we do not require the hiding property.
pub trait PolynomialCommitmentScheme<E: PairingEngine> {
    // Parameters
    type ProverParam;
    type VerifierParam;
    type SRS;
    // Polynomial and its associated types
    type Polynomial;
    type Point;
    type Evaluation;
    // Commitments and proofs
    type Commitment;
    type Proof;
    type BatchProof;
    type BatchCommitment;

    /// Build SRS for testing.
    ///
    /// - For univariate polynomials, `log_size` is the log of maximum degree.
    /// - For multilinear polynomials, `log_size` is the number of variables.
    ///
    /// WARNING: THIS FUNCTION IS FOR TESTING PURPOSE ONLY.
    /// THE OUTPUT SRS SHOULD NOT BE USED IN PRODUCTION.
    fn gen_srs_for_testing<R: RngCore>(
        rng: &mut R,
        log_size: usize,
    ) -> Result<Self::SRS, PCSErrors>;

    /// Generate a commitment for a polynomial
    fn commit(
        prover_param: &Self::ProverParam,
        poly: &Self::Polynomial,
    ) -> Result<Self::Commitment, PCSErrors>;

    /// Generate a commitment for a list of polynomials
    fn multi_commit(
        prover_param: &Self::ProverParam,
        polys: &[Self::Polynomial],
    ) -> Result<Self::BatchCommitment, PCSErrors>;

    /// On input a polynomial `p` and a point `point`, outputs a proof for the
    /// same.
    fn open(
        prover_param: &Self::ProverParam,
        polynomial: &Self::Polynomial,
        point: &Self::Point,
    ) -> Result<(Self::Proof, Self::Evaluation), PCSErrors>;

    /// Input a list of MLEs, and a same number of points, and a transcript,
    /// compute a multi-opening for all the polynomials.
    fn multi_open(
        prover_param: &Self::ProverParam,
        multi_commitment: &Self::Commitment,
        polynomials: &[Self::Polynomial],
        points: &[Self::Point],
    ) -> Result<(Self::BatchProof, Vec<Self::Evaluation>), PCSErrors>;

    /// Verifies that `value` is the evaluation at `x` of the polynomial
    /// committed inside `comm`.
    fn verify(
        verifier_param: &Self::VerifierParam,
        commitment: &Self::Commitment,
        point: &Self::Point,
        value: &E::Fr,
        proof: &Self::Proof,
    ) -> Result<bool, PCSErrors>;

    /// Verifies that `value_i` is the evaluation at `x_i` of the polynomial
    /// `poly_i` committed inside `comm`.
    fn batch_verify<R: RngCore>(
        verifier_param: &Self::VerifierParam,
        multi_commitment: &Self::BatchCommitment,
        points: &[Self::Point],
        values: &[E::Fr],
        batch_proof: &Self::BatchProof,
        rng: &mut R,
    ) -> Result<bool, PCSErrors>;
}

/// API definitions for structured reference string
pub trait StructuredReferenceString<E: PairingEngine>: Sized {
    type ProverParam;
    type VerifierParam;

    /// Extract the prover parameters from the public parameters.
    fn extract_prover_param(&self, supported_log_size: usize) -> Self::ProverParam;
    /// Extract the verifier parameters from the public parameters.
    fn extract_verifier_param(&self, supported_log_size: usize) -> Self::VerifierParam;

    /// Trim the universal parameters to specialize the public parameters
    /// for polynomials to the given `supported_log_size`, and
    /// returns committer key and verifier key.
    ///
    /// - For univariate polynomials, `supported_log_size` is the log of maximum
    ///   degree.
    /// - For multilinear polynomials, `supported_log_size` is the number of
    ///   variables.
    ///
    /// `supported_log_size` should be in range `1..=params.log_size`
    fn trim(
        &self,
        supported_log_size: usize,
    ) -> Result<(Self::ProverParam, Self::VerifierParam), PCSErrors>;

    /// Build SRS for testing.
    ///
    /// - For univariate polynomials, `log_size` is the log of maximum degree.
    /// - For multilinear polynomials, `log_size` is the number of variables.
    ///
    /// WARNING: THIS FUNCTION IS FOR TESTING PURPOSE ONLY.
    /// THE OUTPUT SRS SHOULD NOT BE USED IN PRODUCTION.
    fn gen_srs_for_testing<R: RngCore>(rng: &mut R, log_size: usize) -> Result<Self, PCSErrors>;
}
