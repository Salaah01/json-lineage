class BinaryExecutionException(RuntimeError):
    def __str__(self) -> str:
        return f"Error in calling binary subprocess: {self.args[0]}"
