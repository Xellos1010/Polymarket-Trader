# authentication

```
#include <iostream>
#include <sstream>
#include <string>
#include <cstdlib>
#include <cstring>
#include <chrono>
#include <random>
#include <sodium.h>
#include <nlohmann/json.hpp>
// Base64 URL encoding helper
std::string base64url_encode(const unsigned char* data, size_t len) {
    static const char* base64_chars = 
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    
    std::string encoded;
    encoded.reserve(((len + 2) / 3) * 4);
    
    for (size_t i = 0; i < len; i += 3) {
        unsigned int octet1 = data[i];
        unsigned int octet2 = (i + 1 < len) ? data[i + 1] : 0;
        unsigned int octet3 = (i + 2 < len) ? data[i + 2] : 0;
        
        unsigned int combined = (octet1 << 16) | (octet2 << 8) | octet3;
        
        encoded += base64_chars[(combined >> 18) & 0x3F];
        encoded += base64_chars[(combined >> 12) & 0x3F];
        if (i + 1 < len) encoded += base64_chars[(combined >> 6) & 0x3F];
        if (i + 2 < len) encoded += base64_chars[combined & 0x3F];
    }
    
    return encoded;
}
// Base64 decode helper
std::vector<unsigned char> base64_decode(const std::string& encoded) {
    static const unsigned char base64_table[256] = {
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 62, 64, 64, 64, 63,
        52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 64, 64, 64, 64, 64, 64,
        64,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 64, 64, 64, 64, 64,
        64, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64
    };
    
    std::vector<unsigned char> decoded;
    decoded.reserve((encoded.length() * 3) / 4);
    
    for (size_t i = 0; i < encoded.length(); ) {
        unsigned char c1 = base64_table[static_cast<unsigned char>(encoded[i++])];
        unsigned char c2 = base64_table[static_cast<unsigned char>(encoded[i++])];
        unsigned char c3 = (i < encoded.length()) ? base64_table[static_cast<unsigned char>(encoded[i++])] : 64;
        unsigned char c4 = (i < encoded.length()) ? base64_table[static_cast<unsigned char>(encoded[i++])] : 64;
        
        if (c1 == 64 || c2 == 64) break;
        
        decoded.push_back((c1 << 2) | (c2 >> 4));
        if (c3 != 64) decoded.push_back((c2 << 4) | (c3 >> 2));
        if (c4 != 64) decoded.push_back((c3 << 6) | c4);
    }
    
    return decoded;
}
std::string create_jwt() {
    // Initialize libsodium
    if (sodium_init() < 0) {
        throw std::runtime_error("Failed to initialize libsodium");
    }
    // Fetching environment variables
    const char* key_id_env = std::getenv("KEY_ID");
    const char* key_secret_env = std::getenv("KEY_SECRET");
    const char* request_method_env = std::getenv("REQUEST_METHOD");
    const char* request_host_env = std::getenv("REQUEST_HOST");
    const char* request_path_env = std::getenv("REQUEST_PATH");
    // Ensure all environment variables are present
    if (!key_id_env || !key_secret_env || !request_method_env || !request_host_env || !request_path_env) {
        throw std::runtime_error("Missing required environment variables");
    }
    std::string key_id = key_id_env;
    std::string key_secret = key_secret_env;
    std::string request_method = request_method_env;
    std::string request_host = request_host_env;
    std::string request_path = request_path_env;
    
    // Decode the Ed25519 private key from base64
    std::vector<unsigned char> decoded = base64_decode(key_secret);
    
    // Ed25519 keys are 64 bytes (32 bytes seed + 32 bytes public key)
    if (decoded.size() != 64) {
        throw std::runtime_error("Invalid Ed25519 key length");
    }
    
    // Extract the seed (first 32 bytes)
    unsigned char private_key[32];
    std::memcpy(private_key, decoded.data(), 32);
    
    std::string uri = request_method + " " + request_host + request_path;
    // Generate a random nonce (16 digits)
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, 9);
    std::string nonce;
    for (int i = 0; i < 16; ++i) {
        nonce += std::to_string(dis(gen));
    }
    // Get current timestamp
    auto now = std::chrono::system_clock::now();
    auto now_seconds = std::chrono::duration_cast<std::chrono::seconds>(now.time_since_epoch()).count();
    // Create JWT header
    nlohmann::json header = {
        {"alg", "EdDSA"},
        {"typ", "JWT"},
        {"kid", key_id},
        {"nonce", nonce}
    };
    // Create JWT payload
    nlohmann::json payload = {
        {"sub", key_id},
        {"iss", "cdp"},
        {"aud", nlohmann::json::array({"cdp_service"})},
        {"nbf", now_seconds},
        {"exp", now_seconds + 120},
        {"uri", uri}
    };
    // Encode header and payload
    std::string header_json = header.dump();
    std::string payload_json = payload.dump();
    
    std::string encoded_header = base64url_encode(
        reinterpret_cast<const unsigned char*>(header_json.c_str()), 
        header_json.length()
    );
    std::string encoded_payload = base64url_encode(
        reinterpret_cast<const unsigned char*>(payload_json.c_str()), 
        payload_json.length()
    );
    
    // Create message to sign
    std::string message = encoded_header + "." + encoded_payload;
    
    // Sign with Ed25519
    unsigned char signature[crypto_sign_BYTES];
    crypto_sign_detached(signature, nullptr,
        reinterpret_cast<const unsigned char*>(message.c_str()), message.length(),
        private_key);
    
    // Encode signature
    std::string encoded_signature = base64url_encode(signature, crypto_sign_BYTES);
    
    // Return complete JWT
    return message + "." + encoded_signature;
}
int main() {
    try {
        std::string token = create_jwt();
        std::cout << token << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}

```