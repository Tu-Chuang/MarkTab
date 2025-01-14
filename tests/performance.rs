use actix_web::{test, web, App};
use criterion::{criterion_group, criterion_main, Criterion};
use MARKTAB::{
    controllers,
    models::user::{NewUser, User},
    services::auth::AuthService,
    tests::setup_test_db,
};
use serde_json::json;

async fn setup_test_app() -> (actix_web::test::TestApp, sqlx::MySqlPool) {
    let pool = setup_test_db().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(controllers::config)
    ).await;

    (app, pool)
}

fn bench_login(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let (app, pool) = rt.block_on(async {
        let (app, pool) = setup_test_app().await;
        
        // 创建测试用户
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            password: bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap(),
        };
        User::create(&pool, &new_user).await.unwrap();
        
        (app, pool)
    });

    c.bench_function("login", |b| {
        b.iter(|| {
            rt.block_on(async {
                let req = test::TestRequest::post()
                    .uri("/auth/login")
                    .set_json(json!({
                        "email": "test@example.com",
                        "password": "password123"
                    }))
                    .to_request();

                let resp = test::call_service(&app, req).await;
                assert!(resp.status().is_success());
            });
        });
    });
}

fn bench_get_profile(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let (app, pool) = rt.block_on(async {
        let (app, pool) = setup_test_app().await;
        
        // 创建测试用户并获取token
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            password: bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap(),
        };
        User::create(&pool, &new_user).await.unwrap();
        
        let token = AuthService::login(
            &pool,
            "test@example.com",
            "password123",
            "test",
            "127.0.0.1",
        ).await.unwrap();
        
        (app, token.access_token)
    });

    c.bench_function("get_profile", |b| {
        b.iter(|| {
            rt.block_on(async {
                let req = test::TestRequest::get()
                    .uri("/user/profile")
                    .header("Authorization", format!("Bearer {}", token))
                    .to_request();

                let resp = test::call_service(&app, req).await;
                assert!(resp.status().is_success());
            });
        });
    });
}

criterion_group!(benches, bench_login, bench_get_profile);
criterion_main!(benches); 