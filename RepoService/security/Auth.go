package security

import (
	"fmt"
	"net/http"
	"os"
	"strings"

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

func ParseTokenFromRequest(r *http.Request) (*jwt.Token, error) {
	if r.Header["Authorization"] == nil {
		err := fmt.Errorf("no authorization header found")
		return nil, err
	}

	authHeader := r.Header["Authorization"][0]
	bearer := strings.Split(authHeader, " ")

	secretKey := os.Getenv("SECRET_KEY")
	var mySigningKey = []byte(secretKey)

	token, err := jwt.Parse(bearer[1], func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("there was an error in parsing")
		}
		return mySigningKey, nil
	})

	if err != nil {
		err := fmt.Errorf("your token has expired")
		return nil, err
	}

	return token, err
}
