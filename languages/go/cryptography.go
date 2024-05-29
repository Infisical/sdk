package sdk

type CryptographyInterface interface {
	CreateSymmetricKey() (string, error)
	SymmetricEncrypt(options EncryptSymmetricOptions) (EncryptSymmetricResponse, error)
	SymmetricDecrypt(options DecryptSymmetricOptions) (string, error)
}

type Cryptography struct {
	CommandRunner CommandRunnerInterface
}

func NewCryptography(commandRunner CommandRunnerInterface) *Cryptography {
	return &Cryptography{CommandRunner: commandRunner}
}

func (s *Cryptography) executeCommand(command Command, target interface{}) error {
	responseStr, err := s.CommandRunner.RunCommand(command)
	if err != nil {
		return err
	}
	return checkSuccessAndError(responseStr, target)
}

func (c *Cryptography) CreateSymmetricKey() (string, error) {
	command := Command{
		CreateSymmetricKey: &ArbitraryOptions{},
	}

	var response CreateSymmetricKeyResponse

	if err := c.executeCommand(command, &response); err != nil {
		return "", err
	}

	return response.Key, nil
}

func (c *Cryptography) SymmetricEncrypt(options EncryptSymmetricOptions) (EncryptSymmetricResponse, error) {
	command := Command{
		EncryptSymmetric: &options,
	}

	var response EncryptSymmetricResponse

	if err := c.executeCommand(command, &response); err != nil {
		return EncryptSymmetricResponse{}, err
	}

	return response, nil
}

func (c *Cryptography) SymmetricDecrypt(options DecryptSymmetricOptions) (string, error) {
	command := Command{
		DecryptSymmetric: &options,
	}

	var response DecryptSymmetricResponse

	if err := c.executeCommand(command, &response); err != nil {
		return "", err
	}

	return response.Decrypted, nil
}
