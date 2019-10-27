use crate::{Identity, Qaul, utils, User, UserProfile, QaulResult, QaulError};

/// A random authentication token
pub type Token = String;

/// Wrapper to encode `User` authentication state
///
/// This structure can be aquired by challenging an authentication
/// endpoint, such as `User::login` to yield a token. If a session for
/// this `Identity` already exists, it will be re-used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserAuth(pub Identity, pub(crate) Token);

/// API scope type to access user functions
///
/// Used entirely to namespace API endpoints on `Qaul` instance,
/// without having long type identifiers.
pub struct Users<'chain> {
    pub(crate) q: &'chain Qaul,
}

impl<'qaul> Users<'qaul> {
    /// Drop this scope and return back to global `Qaul` scope
    pub fn drop(&'qaul self) -> &'qaul Qaul {
        self.q
    }

    /// Enumerate locally registered users available
    ///
    /// No information about sessions or existing login state is
    /// stored or accessible via this API.
    pub fn list(&self) -> Vec<UserProfile> {
        self.q.users.get_local()
    }

    /// Create a new user and authenticated session
    ///
    /// The specified password `pw` is used to encrypt the user's
    /// private key and message stores and should be kept safe from
    /// potential attackers.
    ///
    /// It's mandatory to choose a password here, however it is
    /// possible for a frontend to choose a random sequence _for_ a
    /// user, instead of leaving files completely unencrypted. In this
    /// case, there's no real security, but a drive-by will still only
    /// grab encrypted files.
    pub fn create(&self, pw: &str) -> QaulResult<UserAuth> {
        let id = Identity::truncate(&utils::random(16));
        let user = User::Local(UserProfile::new(id));

        // Inform Router about new local user
        self.q.router.local(id);
        
        self.q.users.add_user(user);
        self.q.auth.set_pw(id, pw);
        self.q.auth.new_login(id, pw).map(|t| UserAuth(id, t))
    }

    /// Change the passphrase for an authenticated user
    pub fn change_pw(&self, user: UserAuth, newpw: &str) -> QaulResult<()> {
        let (id, _) = self.q.auth.trusted(user)?;
        self.q.auth.set_pw(id, newpw);
        Ok(())
    }

    /// Create a new session login for a local User
    pub fn login(&self, user: Identity, pw: &str) -> QaulResult<UserAuth> {
        let token = self.q.auth.new_login(user, pw)?;
        Ok(UserAuth(user, token))
    }

    /// Drop the current session Token, invalidating it
    pub fn logout(&self, user: UserAuth) -> QaulResult<()> {
        let (ref id, ref token) = self.q.auth.trusted(user)?;
        self.q.auth.logout(id, token)
    }

    /// Fetch the `UserProfile` for a known identity, remote or local
    ///
    /// No athentication is required for this endpoint, seeing as only
    /// public information is exposed via the `UserProfile`
    /// abstraction anyway.
    pub fn get(&self, user: Identity) -> QaulResult<UserProfile> {
        self.q.users.get(&user)
    }
}
