package request

type HasAccessRequest struct {
	User       string `validate:"required,email"`
	Owner      string `validate:"required,email"`
	SourceName string `validate:"required,source_name"`
}
