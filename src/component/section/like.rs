use yew::prelude::*;

#[function_component(Like)]
pub fn like() -> Html {
    return html! {
        <section id="like" class="blog-mf sect-pt4 route" style="text-align: center;">
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="title-box text-center">
                            <h3 class="title-a">
                                {"愛好"}
                            </h3>
                            <div class="line-mf"></div>
                        </div>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-4">
                        <div class="card card-blog">
                            <div class="card-img">
                                <img src="assets/img/like-1.webp" alt="" class="img-fluid lazyload" />
                            </div>
                            <div class="card-body">
                                <div class="card-category-box">
                                    <div class="card-category">
                                        <h6 class="category">{"品牌"}</h6>
                                    </div>
                                </div>
                                <h3 class="card-title">{"Apple"}</h3>
                                <p class="card-description">
                                    {"我從小學就開始接觸Apple生態圈，非常喜歡Apple的設計理念，甚至將自己的Acer筆電灌入OS X Yosemite，只為了那個美麗的使用者介面與易於操作的系統。<br/><br/>我的所有專案及作品皆運用Apple的設計理念及語言製作，希望能給用戶和Apple產品一樣的體驗。"}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="card card-blog">
                            <div class="card-img">
                                <img src="assets/img/like-2.webp" alt="" class="img-fluid lazyload" />
                            </div>
                            <div class="card-body">
                                <div class="card-category-box">
                                    <div class="card-category">
                                        <h6 class="category">{"國家"}</h6>
                                    </div>
                                </div>
                                <h3 class="card-title">{"日本"}</h3>
                                <p class="card-description">
                                    {"在我國二時，家人帶我去過一次大阪旅遊，從此開啓了對這個東北亞國家的興趣。"}<br/><br/>{"這個國家，在歷史上曾有着不同的面貌，過去是外敵、侵略者或模仿者，現在則是亞洲經濟前段班、文化輸出國及臺灣人愛去的觀光目的地。"}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="card card-blog">
                            <div class="card-img">
                                <img src="assets/img/like-3.webp" alt="" class="img-fluid lazyload" />
                            </div>
                            <div class="card-body">
                                <div class="card-category-box">
                                    <div class="card-category">
                                        <h6 class="category">{"音樂"}</h6>
                                    </div>
                                </div>
                                <h3 class="card-title">{"Japan Pop"}</h3>
                                <p class="card-description">
                                    {"ClariS、Aimer、ChouCho跟LiSA"}<br/><br/>{"可能有些人有聽過他們的名字，因爲他們都是日本有名的動漫歌手，但我是直接去聽他們的歌而知道他們的。"}<br/><br/>{"工作時聽到他們的歌聲總是會讓我充滿活力呢！"}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    };
}
