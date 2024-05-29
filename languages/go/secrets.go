package sdk

type SecretsInterface interface {
	Create(options CreateSecretOptions) (*CreateSecretResponseSecret, error)
	List(options ListSecretsOptions) (*[]SecretElement, error)
	Get(options GetSecretOptions) (*GetSecretResponseSecret, error)
	Delete(options DeleteSecretOptions) (*DeleteSecretResponseSecret, error)
	Update(options UpdateSecretOptions) (*UpdateSecretResponseSecret, error)
}

type Secrets struct {
	CommandRunner CommandRunnerInterface
}

func NewSecrets(commandRunner CommandRunnerInterface) *Secrets {
	return &Secrets{CommandRunner: commandRunner}
}

func (s *Secrets) executeCommand(command Command, target interface{}) error {
	responseStr, err := s.CommandRunner.RunCommand(command)
	if err != nil {
		return err
	}
	return checkSuccessAndError(responseStr, target)
}

func (s *Secrets) Create(options CreateSecretOptions) (*CreateSecretResponseSecret, error) {
	command := Command{
		CreateSecret: &options,
	}

	var response CreateSecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}

	return &response.Secret, nil
}

func (s *Secrets) List(options ListSecretsOptions) (*[]SecretElement, error) {
	command := Command{
		ListSecrets: &options,
	}

	var response ListSecretsResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}

	return &response.Secrets, nil
}

func (s *Secrets) Get(options GetSecretOptions) (*GetSecretResponseSecret, error) {
	command := Command{
		GetSecret: &options,
	}

	var response GetSecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}

	return &response.Secret, nil
}

func (s *Secrets) Delete(options DeleteSecretOptions) (*DeleteSecretResponseSecret, error) {
	command := Command{
		DeleteSecret: &options,
	}

	var response DeleteSecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}

	return &response.Secret, nil
}

func (s *Secrets) Update(options UpdateSecretOptions) (*UpdateSecretResponseSecret, error) {
	command := Command{
		UpdateSecret: &options,
	}

	var response UpdateSecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}

	return &response.Secret, nil
}
