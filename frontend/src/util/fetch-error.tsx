import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

export default class FetchError {
    static toastIfError(response: Response): void {
        if (response.ok) {
            return;
        }

        switch (response.status) {
            case 401:
                toast("401 Unauthorised");
                return;
            case 404:
                toast("404 Not Found");
                return;
            case 500:
                toast("500 Internal Server Error");
                return;
            default:
                toast("404 Not Found");
                return;
        }
    }
}
