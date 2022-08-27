package request

type HasAccessRequest struct {
	User       string `validate:"required,email" json:"user"`
	Owner      string `validate:"required,email" json:"owner"`
	SourceName string `validate:"required,source_name" json:"sourceName"`
}
