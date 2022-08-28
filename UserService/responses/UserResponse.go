package responses

type SearchResult struct {
	Email string `validate:"required,email" mapper:"Email" json:"email"`
}
