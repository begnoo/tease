package security

import (
	"os"
	"time"

	"github.com/golang-jwt/jwt/v4"
)

type Credidentials struct {
	Email    string `validate:"required,email"`
	Password string `validate:"required"`
}

type Token struct {
	Role        string `json:"role"`
	Email       string `json:"email"`
	TokenString string `json:"token"`
}

func GenerateJWT(email, role string, gateway bool) (string, error) {
	var secretKey string
	if gateway {
		secretKey = os.Getenv("BACKEND_SECRET_KEY")
	} else {
		secretKey = os.Getenv("SECRET_KEY")
	}
	var mySigningKey = []byte(secretKey)
	token := jwt.New(jwt.SigningMethodHS256)
	claims := token.Claims.(jwt.MapClaims)

	claims["authorized"] = true
	claims["email"] = email
	claims["role"] = role
	claims["exp"] = time.Now().Add(time.Minute * 30).Unix()

	tokenString, err := token.SignedString(mySigningKey)

	if err != nil {
		return "", err
	}
	return tokenString, nil
}
