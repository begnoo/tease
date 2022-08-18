package request

type CreateSourceRequest struct {
	Name       string `validate:"required,source_name"`
	Owner      string `validate:"required,email"`
	Visability bool
}
