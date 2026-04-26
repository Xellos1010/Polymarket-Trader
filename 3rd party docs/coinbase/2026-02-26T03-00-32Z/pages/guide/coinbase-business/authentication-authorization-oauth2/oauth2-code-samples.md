# oauth2 code samples

```
package main
import (
	"crypto/rand"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"net/url"
)
// Replace these with your Coinbase Business credentials and desired configuration.
var (
	clientID     = "YOUR_CLIENT_ID"
	clientSecret = "YOUR_CLIENT_SECRET"
	// Make sure your redirect URI is registered with your OAuth2 client.
	redirectURI = "http://localhost:8000/callback"
	// Coinbase OAuth2 endpoints
	authURL  = "https://login.coinbase.com/oauth2/auth"
	tokenURL = "https://login.coinbase.com/oauth2/token"
)
// TokenResponse models the response received after exchanging the code.
type TokenResponse struct {
	AccessToken  string `json:"access_token"`
	TokenType    string `json:"token_type"`
	ExpiresIn    int    `json:"expires_in"`
	RefreshToken string `json:"refresh_token"`
	Scope        string `json:"scope"`
}
func main() {
	http.HandleFunc("/login", loginHandler)
	http.HandleFunc("/callback", callbackHandler)
	log.Println("Starting server on :8000")
	log.Fatal(http.ListenAndServe(":8000", nil))
}
// loginHandler initiates the OAuth2 flow.
func loginHandler(w http.ResponseWriter, r *http.Request) {
	state, err := generateState()
	if err != nil {
		http.Error(w, "Error generating state", http.StatusInternalServerError)
		return
	}
	// For demo purposes, we store the state in a cookie.
	http.SetCookie(w, &http.Cookie{
		Name:  "oauthstate",
		Value: state,
		Path:  "/",
	})
	// Build the authorization URL.
	params := url.Values{}
	params.Add("response_type", "code")
	params.Add("client_id", clientID)
	params.Add("redirect_uri", redirectURI)
	// Adjust the scope as needed per Coinbase’s documentation.
	params.Add("scope", "wallet:user:read")
	params.Add("state", state)
	http.Redirect(w, r, authURL+"?"+params.Encode(), http.StatusFound)
}
// callbackHandler handles the OAuth2 callback and exchanges the code for an access token.
func callbackHandler(w http.ResponseWriter, r *http.Request) {
	// Retrieve state and code from the query parameters.
	query := r.URL.Query()
	state := query.Get("state")
	code := query.Get("code")
	// Validate the state using the stored cookie.
	cookie, err := r.Cookie("oauthstate")
	if err != nil || cookie.Value != state {
		http.Error(w, "Invalid state", http.StatusBadRequest)
		return
	}
	// Exchange the code for an access token.
	token, err := exchangeCodeForToken(code)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error exchanging token: %v", err), http.StatusInternalServerError)
		return
	}
	// For demo purposes, we simply output the access token.
	fmt.Fprintf(w, "Access token: %s\n", token.AccessToken)
	fmt.Fprintf(w, "Refresh token: %s\n", token.RefreshToken)
}
// exchangeCodeForToken makes a POST request to Coinbase's token endpoint to exchange the authorization code for an access token.
func exchangeCodeForToken(code string) (*TokenResponse, error) {
	data := url.Values{}
	data.Set("grant_type", "authorization_code")
	data.Set("code", code)
	data.Set("redirect_uri", redirectURI)
	data.Set("client_id", clientID)
	data.Set("client_secret", clientSecret)
	resp, err := http.PostForm(tokenURL, data)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	var tokenRes TokenResponse
	if err := json.NewDecoder(resp.Body).Decode(&tokenRes); err != nil {
		return nil, err
	}
	return &tokenRes, nil
}
// generateState creates a random string to be used as the OAuth state parameter.
func generateState() (string, error) {
	b := make([]byte, 16)
	_, err := rand.Read(b)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("%x", b), nil
}

```