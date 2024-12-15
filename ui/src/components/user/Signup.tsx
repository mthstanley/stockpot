import {useNavigate} from "react-router";
import {useAuth} from "../AuthProvider";
import {apiClient} from "../../utils/api";

const SignupPage = () => {
    const navigate = useNavigate();
    const auth = useAuth();

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();

        const formData = new FormData(event.currentTarget);
        const name = formData.get("name") as string;
        const username = formData.get("username") as string;
        const password = formData.get("password") as string;

        apiClient.createUser({name, username, password});
        auth.signin(username, password, () => {
            // Send them back to the page they tried to visit when they were
            // redirected to the login page. Use { replace: true } so we don't create
            // another entry in the history stack for the login page.  This means that
            // when they get to the protected page and click the back button, they
            // won't end up back on the login page, which is also really nice for the
            // user experience.
            navigate("/users");
        });
    }

    return (
        <div>
            <h1>Sign-Up</h1>

            <form onSubmit={handleSubmit}>
                <label>
                    Name: <input name="name" type="text" />
                </label>{" "}
                <label>
                    Username: <input name="username" type="text" />
                </label>{" "}
                <label>
                    Password: <input name="password" type="text" />
                </label>{" "}
                <button type="submit">Sign-up</button>
            </form>
        </div>
    );
};

export default SignupPage
