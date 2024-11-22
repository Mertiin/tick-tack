interface Props {
  title: string;
  description: string;
}

const LoginHeader = ({ title, description }: Props) => {
  return (
    <div className="space-y-2 text-center">
      <h1 className="text-2xl">{title}</h1>
      <span className="text-sm text-primary/60">{description}</span>
    </div>
  );
};

export { LoginHeader };
